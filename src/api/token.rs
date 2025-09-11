use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug, AsRefStr, Hash, Eq)]
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
    POL,
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
    TAL,
    DOT,
    NFT,
    SOL,
    XAUT,
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
            "POL" => Ok(Token::POL),
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
            "TAL" => Ok(Token::TAL),
            "DOT" => Ok(Token::DOT),
            "NFT" => Ok(Token::NFT),
            "SOL" => Ok(Token::SOL),
            "XAUT" => Ok(Token::XAUT),
            token => Err(TokenError::InvalidToken(token.to_owned())),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
#[derive(Debug)]
pub enum TokenError {
    InvalidToken(String),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct CurrencyPair(pub [Token; 2]);

impl CurrencyPair {
    pub fn new(first: Token, second: Token) -> Self {
        Self([first, second])
    }

    pub fn first(&self) -> Token {
        self.0[0]
    }

    pub fn second(&self) -> Token {
        self.0[1]
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
        format!("{}/{}", value.0[0].as_ref(), value.0[1].as_ref())
    }
}

impl Display for CurrencyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0[0].as_ref(), self.0[1].as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_conversion() {
        let token: Token = Token::ADA;
        let token_str: &str = token.as_ref();
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
