use std::{default, str::FromStr};

pub enum HTTPMethod {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for HTTPMethod {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(SELF::GET),
            "DELETE" => Ok(SELF::GET),
            "POST" => Ok(SELF::POST),
            "PUT" => Ok(SELF::PUT),
            "HEAD" => Ok(SELF::HEAD),
            "CONNECT" => Ok(SELF::CONNECT),
            "OPTIONS" => Ok(SELF::OPTIONS),
            "TRACE" => Ok(SELF::TRACE),
            "PATCH" => Ok(SELF::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
