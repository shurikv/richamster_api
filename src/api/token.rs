use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug, AsRefStr, Hash, Eq, strum_macros::Display, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
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
    WLD,
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
