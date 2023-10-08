mod menu;

use crate::menu::{Menu, MenuItems};
use richamster_api::errors::RichamsterError;
use richamster_api::models::authentication::{LoginResponse, OtpLoginResponse};
use richamster_api::models::user::TransactionsFilter;
use richamster_api::richamster::auth::Auth;
use richamster_api::richamster::user::User;
use std::fmt::Error;
use std::io::BufRead;
use tracing::subscriber::set_global_default;
use tracing::{error, info};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

fn setup_logger() {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json());
    set_global_default(subscriber).expect("Failed to set subscriber");

    info!("Logger initialized");
}

pub struct JwtTokenStorage {
    token: Option<String>,
}

impl JwtTokenStorage {
    pub fn update_token(&mut self, token: String) {
        self.token = Some(token);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger();

    let mut token_storage = JwtTokenStorage { token: None };

    loop {
        Menu::print();
        println!("Select your choice: ");
        let input = read_input();
        let item = input.trim().parse::<usize>();
        if item.is_err() {
            info!("Invalid input");
            continue;
        }
        let item: MenuItems = item.unwrap().into();
        if item != MenuItems::Quit {
            Menu::print_header(&item);
        }
        match item {
            MenuItems::Login => {
                let login_result = login().await;
                match login_result {
                    Ok(token) => token_storage.update_token(token),
                    Err(e) => {
                        error!("Authentication error: {:?}", e)
                    }
                }
            }
            MenuItems::UserDetails => {
                let _ = show_user_details(&token_storage).await;
            }
            MenuItems::UserBalance => {
                let _ = show_user_balance(&token_storage).await;
            }
            MenuItems::UserTransactions => {
                let _ = show_user_transactions(&token_storage).await;
            }
            MenuItems::Quit => break,
        }
    }
    Ok(())
}

async fn login() -> Result<String, RichamsterError> {
    println!("Enter your username: ");
    let username = read_input();
    println!("Enter your password: ");
    let password = read_input();
    let auth = Auth::login(username, password).await;
    match auth {
        Ok(login_response) => match login_response {
            LoginResponse::Jwt(token) => Ok(token),
            LoginResponse::RequiresTwoFactor(_) => {
                println!("Enter otp code: ");
                let otp_code = read_input();
                let otp = Auth::two_factor_login(otp_code).await;
                match otp {
                    Ok(otp_response) => match otp_response {
                        OtpLoginResponse::Jwt(token) => Ok(token),
                        OtpLoginResponse::Error(e) => {
                            error!("{}", e);
                            Err(RichamsterError::Otp(e))
                        }
                    },
                    Err(e) => {
                        error!("{:?}", e);
                        Err(e)
                    }
                }
            }
        },
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}

async fn show_user_details(token_storage: &JwtTokenStorage) -> Result<(), RichamsterError> {
    if token_storage.token.is_none() {
        return Err(RichamsterError::UnauthorizedAccess);
    }
    let user = User::with_jwt_token(token_storage.token.clone().unwrap());
    let result = user.detail_info().await?;
    println!("{}", result);
    Ok(())
}

async fn show_user_balance(token_storage: &JwtTokenStorage) -> Result<(), RichamsterError> {
    if token_storage.token.is_none() {
        return Err(RichamsterError::UnauthorizedAccess);
    }
    let user = User::with_jwt_token(token_storage.token.clone().unwrap());
    let result = user.balances(None).await?;
    for balance in result {
        println!(
            "{:?}: {:?}: {}",
            balance.currency.abbreviation, balance.active_balance, balance.balance
        );
    }
    Ok(())
}

async fn show_user_transactions(token_storage: &JwtTokenStorage) -> Result<(), RichamsterError> {
    if token_storage.token.is_none() {
        return Err(RichamsterError::UnauthorizedAccess);
    }
    let user = User::with_jwt_token(token_storage.token.clone().unwrap());
    let result = user
        .transactions_list(TransactionsFilter {
            currency: None,
            transaction_type: None,
            closed_at_gte: None,
            closed_at_lte: None,
        })
        .await?;
    println!("{:?}", result);
    Ok(())
}

fn read_input() -> String {
    let mut input = String::new();
    let _ = std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("Failed to get input");
    input.trim().to_owned()
}
