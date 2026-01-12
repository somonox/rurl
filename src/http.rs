pub struct HTTPConnectionInfo {
    method: String,
    scheme: String,
    host: String,
    port: u16,
    path: String,
    query: String,
    header: String,
    body: String,
}

impl From<HTTPConnectionInfo> for NetworkConnectionInfo {
    fn from(value: HTTPConnectionInfo) -> Self {
        Self {
            scheme: value.scheme,
            host: value.host,
            port: value.port,
            path: value.path,
        }
    }
}
