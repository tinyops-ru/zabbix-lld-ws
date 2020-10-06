pub mod zabbix {
    use serde::Serialize;

    pub const JSONRPC: &str = "2.0";

    pub const CONTENT_TYPE_HEADER: &str = "Content-Type";
    pub const CONTENT_TYPE_JSON: &str = "application/json";

    #[derive(Serialize)]
    pub struct ZabbixRequest<P: Serialize> {
        pub jsonrpc: String,
        pub method: String,
        pub params: P,
        pub auth: String,
        pub id: i8
    }
}
