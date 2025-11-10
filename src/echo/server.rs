use actix_web::{web, App, HttpServer};
use rmcp::{
    handler::server::ServerHandler,
    model::*,
    service::{RequestContext, RoleServer},
    ErrorData as McpError
};
use rmcp_actix_web::transport::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use std::sync::Arc;
use std::time::Duration;


#[derive(Clone)]
pub struct EchoServer;

impl ServerHandler for EchoServer;
    fn get_info(&self) -> ServerInfo{
        ServerInfo{
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
            .enable_tools()
            .build(),
            server_info: Implementation {
                name: "echo-mcp".into(),
                version: "0.1.0".into(),
                title: Some("Echo MCP Server".into()),
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "Use the 'echo' tool to send text and get it echoed back.".into(),
            ),
        }
    }