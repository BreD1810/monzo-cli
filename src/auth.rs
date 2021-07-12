use monzo::inner_client::Refreshable;
use monzo::Client;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse,
    TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

#[derive(Default, Serialize, Deserialize)]
pub struct OauthInfo {
    access_token: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl OauthInfo {
    async fn new() -> OauthInfo {
        let mut cfg = get_config();

        if cfg.client_id.is_empty() {
            cfg.client_id = get_client_id();
        }

        if cfg.client_secret.is_empty() {
            cfg.client_secret = get_client_secret();
        }

        let auth_info = do_oauth(&cfg).await;
        auth_info
    }
}

fn save_config(auth_info: &OauthInfo) {
    let res = confy::store("monzo-cli", auth_info);
    match res {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error saving config data");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn get_config() -> OauthInfo {
    let cfg = confy::load("monzo-cli");
    match cfg {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Error loading config");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

pub async fn auth() {
    let auth_info = OauthInfo::new().await;
    save_config(&auth_info);
    println!("Make sure you allow access on your Monzo app!");
}

pub async fn get_authed_client() -> Client<Refreshable> {
    let mut cfg = get_config();
    let mut client = Client::new(cfg.access_token.clone()).with_refresh_tokens(
        cfg.client_id.clone(),
        cfg.client_secret.clone(),
        cfg.refresh_token.clone(),
    );

    let auth_check = client.accounts().await;
    match auth_check {
        Ok(_) => {}
        Err(monzo::Error::AuthExpired) => {
            refresh_client(&mut client, &mut cfg).await;
        }
        Err(e) => match e {
            monzo::Error::Client(code) if code == 403 => {
                eprintln!("Error: {}", e);
                eprintln!("It is likely that you need to allow access in your Monzo app");
                std::process::exit(1);
            }
            _ => {
                eprintln!("Error: {}", e);
                eprintln!("Please run `monzo auth`");
                std::process::exit(1);
            }
        },
    };

    client
}

async fn refresh_client(client: &mut Client<Refreshable>, cfg: &mut OauthInfo) {
    let refresh_result = client.refresh_auth().await;
    match refresh_result {
        Ok(()) => {
            cfg.access_token = client.access_token().to_owned();
            cfg.refresh_token = client.refresh_token().to_owned();
            save_config(&cfg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please run `monzo auth`");
            std::process::exit(1);
        }
    }
}

fn get_client_id() -> String {
    println!("Please enter the client ID:");
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap_or_else(|e| {
        eprintln!("Error reading client ID:\n{}", e);
        std::process::exit(1);
    });
    response.trim_end().to_string()
}

fn get_client_secret() -> String {
    println!("Please enter the client secret:");
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap_or_else(|e| {
        eprint!("Error reading client secret:\n{}", e);
        std::process::exit(1);
    });
    response.trim_end().to_string()
}

async fn do_oauth(auth_info: &OauthInfo) -> OauthInfo {
    let client = BasicClient::new(
        ClientId::new(auth_info.client_id.clone()),
        Some(ClientSecret::new(auth_info.client_secret.clone())),
        AuthUrl::new("https://auth.monzo.com".to_string()).expect("blah"),
        Some(TokenUrl::new("https://api.monzo.com/oauth2/token".to_string()).expect("blah")),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string()).expect("blah"));

    let (auth_url, _) = client.authorize_url(CsrfToken::new_random).url();

    println!("Browse to: {}", auth_url);

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await.unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());
            }

            let message = "Go back to the terminal";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await.unwrap();

            let token_result = client
                .set_auth_type(oauth2::AuthType::RequestBody)
                .exchange_code(code)
                .request_async(async_http_client)
                .await;

            match token_result {
                Ok(token) => {
                    return OauthInfo {
                        access_token: token.access_token().secret().into(),
                        client_id: auth_info.client_id.clone(),
                        client_secret: auth_info.client_secret.clone(),
                        refresh_token: token.refresh_token().unwrap().secret().into(),
                    }
                }
                Err(e) => {
                    eprintln!("Error authorising: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
