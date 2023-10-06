use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum Token {
    FSH,
    TON,
    UAHT,
    AAVE,
    HCK,
    TRX,
    CRO,
    VQR,
    SHIB,
    TLR,
    LINK,
    MATIC,
    UNI,
    USDC,
    BAT,
    USDT,
    RCH,
    BOX,
    XMR,
    DASH,
    KUB,
    WAVES,
    ADA,
    ETH,
    DOGE,
    KRB,
    UAH,
    BTC,
    LTC,
}

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FSH" => Ok(Token::FSH),
            "TON" => Ok(Token::TON),
            "UAHT" => Ok(Token::UAHT),
            "AAVE" => Ok(Token::AAVE),
            "HCK" => Ok(Token::HCK),
            "TRX" => Ok(Token::TRX),
            "CRO" => Ok(Token::CRO),
            "VQR" => Ok(Token::VQR),
            "SHIB" => Ok(Token::SHIB),
            "TLR" => Ok(Token::TLR),
            "LINK" => Ok(Token::LINK),
            "MATIC" => Ok(Token::MATIC),
            "UNI" => Ok(Token::UNI),
            "USDC" => Ok(Token::USDC),
            "BAT" => Ok(Token::BAT),
            "USDT" => Ok(Token::USDT),
            "RCH" => Ok(Token::RCH),
            "BOX" => Ok(Token::BOX),
            "XMR" => Ok(Token::XMR),
            "DASH" => Ok(Token::DASH),
            "KUB" => Ok(Token::KUB),
            "WAVES" => Ok(Token::WAVES),
            "ADA" => Ok(Token::ADA),
            "ETH" => Ok(Token::ETH),
            "DOGE" => Ok(Token::DOGE),
            "KRB" => Ok(Token::KRB),
            "UAH" => Ok(Token::UAH),
            "BTC" => Ok(Token::BTC),
            "LTC" => Ok(Token::LTC),
            token => Err(TokenError::InvalidToken(token.to_owned())),
        }
    }
}

impl From<Token> for &str {
    fn from(value: Token) -> Self {
        match value {
            Token::FSH => "FSH",
            Token::TON => "TON",
            Token::UAHT => "UAHT",
            Token::AAVE => "AAVE",
            Token::HCK => "HCK",
            Token::TRX => "TRX",
            Token::CRO => "CRO",
            Token::VQR => "VQR",
            Token::SHIB => "SHIB",
            Token::TLR => "TLR",
            Token::LINK => "LINK",
            Token::MATIC => "MATIC",
            Token::UNI => "UNI",
            Token::USDC => "USDC",
            Token::BAT => "BAT",
            Token::USDT => "USDT",
            Token::RCH => "RCH",
            Token::BOX => "BOX",
            Token::XMR => "XMR",
            Token::DASH => "DASH",
            Token::KUB => "KUB",
            Token::WAVES => "WAVES",
            Token::ADA => "ADA",
            Token::ETH => "ETH",
            Token::DOGE => "DOGE",
            Token::KRB => "KRB",
            Token::UAH => "UAH",
            Token::BTC => "BTC",
            Token::LTC => "LTC",
        }
    }
}

#[derive(Debug)]
pub enum TokenError {
    InvalidToken(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CurrencyPair([Token; 2]);

impl CurrencyPair {
    pub fn new(first: Token, second: Token) -> Self {
        Self([first, second])
    }
}

#[derive(Debug)]
pub enum CurrencyPairError {
    InvalidToken(String),
    IllegalDelimiterCount(usize),
}

impl FromStr for CurrencyPair {
    type Err = CurrencyPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<Token> = s
            .split('/')
            .map(|v| {
                v.parse::<Token>()
                    .map_err(|_| CurrencyPairError::InvalidToken(v.to_owned()))
            })
            .collect::<Result<_, CurrencyPairError>>()?;
        if split.len() != 2 {
            return Err(CurrencyPairError::IllegalDelimiterCount(split.len()));
        }
        Ok(Self(split.try_into().unwrap()))
    }
}

impl From<CurrencyPair> for String {
    fn from(value: CurrencyPair) -> Self {
        format!(
            "{}/{}",
            <Token as Into<&str>>::into(value.0[0]),
            <Token as Into<&str>>::into(value.0[1])
        )
    }
}

impl Display for CurrencyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}",
            <Token as Into<&str>>::into(self.0[0]),
            <Token as Into<&str>>::into(self.0[1])
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_conversion() {
        let token: Token = Token::ADA;
        let token_str: &str = token.into();
        assert_eq!(token_str, "ADA");
        let token2: Token = token_str.parse().unwrap();
        assert_eq!(token2, Token::ADA);
    }

    #[test]
    fn invalid_string_conversion() {
        let token_str: &str = "APT";
        let result = token_str.parse::<Token>();
        assert!(result.is_err());
    }

    #[test]
    fn string_currency_pair_conversion() {
        let pair: &str = "BTC/UAH";
        let result: Result<CurrencyPair, CurrencyPairError> = pair.parse::<CurrencyPair>();
        assert!(result.is_ok());
    }

    #[test]
    fn lowercase_string_currency_pair_conversion() {
        let pair: &str = "btc/uah";
        let result: Result<CurrencyPair, CurrencyPairError> = pair.parse::<CurrencyPair>();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_string_currency_pair_conversion() {
        let pair: &str = "BTC/APT";
        let result: Result<CurrencyPair, CurrencyPairError> = pair.parse::<CurrencyPair>();
        assert!(result.is_err());
    }

    #[test]
    fn invalid_delimiter_count_string_currency_pair_conversion() {
        let pair: &str = "BTC/UAH/BAT";
        let result: Result<CurrencyPair, CurrencyPairError> = pair.parse::<CurrencyPair>();
        assert!(result.is_err());
    }
}
