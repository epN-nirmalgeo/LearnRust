/*
GET /accounts?acct_id=100 HTTP/1.1 \r\n
HEADERS \r\n
BODY
*/
use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

