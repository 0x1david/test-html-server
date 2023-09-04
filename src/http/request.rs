use std::str::Utf8Error;
use std::str;
use super::method::HTTPMethod;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FmtResult, Debug };

pub struct Request {
        path: String,
        query_string: Option<String>,
        method: HTTPMethod,
    }

impl TryFrom<&[u8]> for Request {
   type Error = ParseError;

   fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        unimplemented!()
   }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
       if c == ' ' || c == '\r' {
           return Some((&request[..i], &request[i + 1..]));
       } 
    }

    None
}

pub enum ParseError {
    InvalidEncoding,
    InvalidMethod,
    InvalidProtocol,
    InvalidRequest,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
        Self::InvalidEncoding => "InvalidEncoding",
        Self::InvalidMethod => "InvalidMethod",
        Self::InvalidProtocol => "InvalidProtocol",
        Self::InvalidRequest => "InvalidRequest",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}


impl Error for ParseError {}

