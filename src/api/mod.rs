use once_cell::sync::Lazy;
use std::str::FromStr;
use url::Url;

pub mod token;

static BASE_URL: Lazy<Url> =
    Lazy::new(|| Url::from_str("https://richamster.com/public/v1/").unwrap());

pub enum Api {
    Exchange(ExchangeApi),
    Feedback(FeedbackApi),
    Authentication(AuthenticationApi),
    Payments(PaymentsApi),
    Transfer(TransferApi),
    User(UserApi),
}

pub enum ExchangeApi {
    Currencies,
    Favourites,
    Markets,
    OrderBook,
    OrdersHistory,
    Restrictions,
    TickerList,
    UserOrders,
    NewOrder,
    MarketOrder,
    ExecuteMarketOrder,
    DestroyOrder,
}

pub enum FeedbackApi {
    ContactUs,
    Messengers,
}

pub enum AuthenticationApi {
    Login,
    Register,
    RefreshToken,
    VerifyToken,
    TwoFactorLogin,
}

pub enum PaymentsApi {
    ReplenishInfo,
    Replenish,
    WithdrawInfo,
    Withdraw,
}

pub enum TransferApi {
    Transfer,
}

pub enum UserApi {
    Balances,
    Detail,
    Orders,
    Transactions,
}

pub trait RequestPath {
    fn path(&self) -> &str;
    fn full_url(&self) -> Url;
}

impl RequestPath for Api {
    fn path(&self) -> &str {
        match self {
            Api::Exchange(exchange) => match exchange {
                ExchangeApi::Currencies => "exchange/currencies/",
                ExchangeApi::Favourites => "exchange/favourites/{id}/toggle/",
                ExchangeApi::Markets => "exchange/markets/",
                ExchangeApi::OrderBook => "exchange/order-book/",
                ExchangeApi::OrdersHistory => "exchange/orders/history/",
                ExchangeApi::Restrictions => "exchange/restrictions/",
                ExchangeApi::TickerList => "exchange/ticker/",
                ExchangeApi::UserOrders => "exchange/user/orders/",
                ExchangeApi::NewOrder => "exchange/user/orders/",
                ExchangeApi::MarketOrder => "exchange/user/orders/market/",
                ExchangeApi::ExecuteMarketOrder => "exchange/user/orders/market/",
                ExchangeApi::DestroyOrder => "exchange/user/orders/{id}/",
            },
            Api::Feedback(feedback) => match feedback {
                FeedbackApi::ContactUs => "feedback/contact-us/",
                FeedbackApi::Messengers => "feedback/messengers/",
            },
            Api::Authentication(authentication) => match authentication {
                AuthenticationApi::Login => "login/",
                AuthenticationApi::Register => "register/",
                AuthenticationApi::RefreshToken => "token/refresh/",
                AuthenticationApi::VerifyToken => "token/verify/",
                AuthenticationApi::TwoFactorLogin => "two-factor-login/",
            },
            Api::Payments(payments) => match payments {
                PaymentsApi::ReplenishInfo => "payments/replenish/{currency}/",
                PaymentsApi::Replenish => "payments/replenish/{currency}/",
                PaymentsApi::WithdrawInfo => "payments/withdraw/{currency}/",
                PaymentsApi::Withdraw => "payments/withdraw/{currency}/",
            },
            Api::Transfer(transfer) => match transfer {
                TransferApi::Transfer => "transfer/create/",
            },
            Api::User(user) => match user {
                UserApi::Balances => "user/balances/",
                UserApi::Detail => "user/detail/",
                UserApi::Orders => "user/orders/",
                UserApi::Transactions => "user/transactions/",
            },
        }
    }

    fn full_url(&self) -> Url {
        BASE_URL.join(self.path()).unwrap()
    }
}
