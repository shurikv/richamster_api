use once_cell::sync::Lazy;
use reqwest::Method;
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
    fn request_data(&self) -> RequestData;
    fn full_url(&self, path: &str) -> Url;
}

pub struct RequestData(pub Url, pub Method);

impl RequestPath for Api {
    fn request_data(&self) -> RequestData {
        match self {
            Api::Exchange(exchange) => match exchange {
                ExchangeApi::Currencies => {
                    RequestData(self.full_url("exchange/currencies/"), Method::GET)
                }
                ExchangeApi::Favourites => RequestData(
                    self.full_url("exchange/favourites/{id}/toggle/"),
                    Method::POST,
                ),
                ExchangeApi::Markets => {
                    RequestData(self.full_url("exchange/markets/"), Method::GET)
                }
                ExchangeApi::OrderBook => {
                    RequestData(self.full_url("exchange/order-book/"), Method::GET)
                }
                ExchangeApi::OrdersHistory => {
                    RequestData(self.full_url("exchange/orders/history/"), Method::GET)
                }
                ExchangeApi::Restrictions => {
                    RequestData(self.full_url("exchange/restrictions/"), Method::GET)
                }
                ExchangeApi::TickerList => {
                    RequestData(self.full_url("exchange/ticker/"), Method::GET)
                }
                ExchangeApi::UserOrders => {
                    RequestData(self.full_url("exchange/user/orders/"), Method::GET)
                }
                ExchangeApi::NewOrder => {
                    RequestData(self.full_url("exchange/user/orders/"), Method::POST)
                }
                ExchangeApi::MarketOrder => {
                    RequestData(self.full_url("exchange/user/orders/market/"), Method::GET)
                }
                ExchangeApi::ExecuteMarketOrder => {
                    RequestData(self.full_url("exchange/user/orders/market/"), Method::POST)
                }
                ExchangeApi::DestroyOrder => {
                    RequestData(self.full_url("exchange/user/orders/"), Method::DELETE)
                }
            },
            Api::Feedback(feedback) => match feedback {
                FeedbackApi::ContactUs => {
                    RequestData(self.full_url("feedback/contact-us/"), Method::POST)
                }
                FeedbackApi::Messengers => {
                    RequestData(self.full_url("feedback/messengers/"), Method::GET)
                }
            },
            Api::Authentication(authentication) => match authentication {
                AuthenticationApi::Login => RequestData(self.full_url("login/"), Method::POST),
                AuthenticationApi::Register => {
                    RequestData(self.full_url("register/"), Method::POST)
                }
                AuthenticationApi::RefreshToken => {
                    RequestData(self.full_url("token/refresh/"), Method::POST)
                }
                AuthenticationApi::VerifyToken => {
                    RequestData(self.full_url("token/verify/"), Method::POST)
                }
                AuthenticationApi::TwoFactorLogin => {
                    RequestData(self.full_url("two-factor-login/"), Method::POST)
                }
            },
            Api::Payments(payments) => match payments {
                PaymentsApi::ReplenishInfo => {
                    RequestData(self.full_url("payments/replenish/"), Method::GET)
                }
                PaymentsApi::Replenish => {
                    RequestData(self.full_url("payments/replenish/"), Method::POST)
                }
                PaymentsApi::WithdrawInfo => {
                    RequestData(self.full_url("payments/withdraw/"), Method::GET)
                }
                PaymentsApi::Withdraw => {
                    RequestData(self.full_url("payments/withdraw/"), Method::POST)
                }
            },
            Api::Transfer(transfer) => match transfer {
                TransferApi::Transfer => {
                    RequestData(self.full_url("transfer/create/"), Method::POST)
                }
            },
            Api::User(user) => match user {
                UserApi::Balances => RequestData(self.full_url("user/balances/"), Method::GET),
                UserApi::Detail => RequestData(self.full_url("user/detail/"), Method::GET),
                UserApi::Orders => RequestData(self.full_url("user/orders/"), Method::GET),
                UserApi::Transactions => {
                    RequestData(self.full_url("user/transactions/"), Method::GET)
                }
            },
        }
    }

    fn full_url(&self, path: &str) -> Url {
        BASE_URL.join(path).unwrap()
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use percent_encoding::percent_decode_str;

    #[test]
    fn exchange_join_path() {
        let req_data = Api::Exchange(ExchangeApi::MarketOrder).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/user/orders/market/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::OrdersHistory).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/orders/history/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::NewOrder).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/user/orders/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Exchange(ExchangeApi::UserOrders).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/user/orders/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::DestroyOrder).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/user/orders/"
        );
        assert_eq!(req_data.1, Method::DELETE);
        let req_data = Api::Exchange(ExchangeApi::OrderBook).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/order-book/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::Restrictions).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/restrictions/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::Currencies).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/currencies/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::Favourites).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/favourites/{id}/toggle/"
        );
        let req_data = Api::Exchange(ExchangeApi::ExecuteMarketOrder).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/user/orders/market/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Exchange(ExchangeApi::TickerList).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/ticker/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Exchange(ExchangeApi::OrderBook).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/exchange/order-book/"
        );
        assert_eq!(req_data.1, Method::GET);
    }

    #[test]
    fn auth_join_path() {
        let req_data = Api::Authentication(AuthenticationApi::Login).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/login/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Authentication(AuthenticationApi::Register).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/register/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Authentication(AuthenticationApi::TwoFactorLogin).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/two-factor-login/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Authentication(AuthenticationApi::RefreshToken).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/token/refresh/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Authentication(AuthenticationApi::VerifyToken).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/token/verify/"
        );
        assert_eq!(req_data.1, Method::POST);
    }

    #[test]
    fn feedback_join_path() {
        let req_data = Api::Feedback(FeedbackApi::ContactUs).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/feedback/contact-us/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Feedback(FeedbackApi::Messengers).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/feedback/messengers/"
        );
        assert_eq!(req_data.1, Method::GET);
    }

    #[test]
    fn user_join_path() {
        let req_data = Api::User(UserApi::Detail).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/user/detail/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::User(UserApi::Balances).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/user/balances/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::User(UserApi::Orders).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/user/orders/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::User(UserApi::Transactions).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/user/transactions/"
        );
        assert_eq!(req_data.1, Method::GET);
    }

    #[test]
    fn payments_join_path() {
        let req_data = Api::Payments(PaymentsApi::Replenish).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/payments/replenish/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Payments(PaymentsApi::Withdraw).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/payments/withdraw/"
        );
        assert_eq!(req_data.1, Method::POST);
        let req_data = Api::Payments(PaymentsApi::ReplenishInfo).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/payments/replenish/"
        );
        assert_eq!(req_data.1, Method::GET);
        let req_data = Api::Payments(PaymentsApi::WithdrawInfo).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/payments/withdraw/"
        );
        assert_eq!(req_data.1, Method::GET);
    }

    #[test]
    fn transfer_join_path() {
        let req_data = Api::Transfer(TransferApi::Transfer).request_data();
        assert_eq!(
            percent_decode_str(req_data.0.as_str()).decode_utf8_lossy(),
            "https://richamster.com/public/v1/transfer/create/"
        );
        assert_eq!(req_data.1, Method::POST);
    }
}
