use actix_web::{web, App, HttpServer};
use rmcp_actix_web::transport::StreamableHttpService
use tokio::sync::Mutex
use rmcp::{
    handler::server::ServerHandler,
    model::*,
    service::{RequestContext, RoleServer},
    ErrorData as McpError,
};
use std::sync::Arc;
use std::time::Duration;

#[derive(clone)]
pub struct EchoHandler;

impl ServerHandler for EchoHandler