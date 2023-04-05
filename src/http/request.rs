use super::method::Method;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str;
use std::str::{Utf8Error};
use std::option::Option;
use crate::http::method::MethodError;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl Request {

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(&value)?;

        let (method,request) = get_next_word(&request).ok_or(ParseError::InvalidRequest)?;
        let (path,request) = get_next_word(&request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(&request).ok_or(ParseError::InvalidRequest)?;

        println!("{} {} {}", &method,&path,&protocol);

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        println!("{} is the value of our METHOD", method);

        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index,item) in request.chars().enumerate() {
        if item == ' ' || item == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest, // bad request
    InvalidEncoding, // encoding of incorrect type
    InvalidProtocol, //only support http 1.1
    InvalidMethod // method that we don't implement,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f,"{}", &self.message())

    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f,"{}", &self.message())
    }
}

impl Error for ParseError {}
