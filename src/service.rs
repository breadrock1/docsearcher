use crate::endpoints::buckets::{
    all_buckets, default_bucket, delete_bucket, get_bucket, new_bucket,
};
use crate::endpoints::clusters::{all_clusters, delete_cluster, get_cluster, new_cluster};
use crate::endpoints::documents::{delete_document, get_document, new_document, update_document};
use crate::endpoints::hello::hello;
use crate::endpoints::loader::{load_file, upload_file};
use crate::endpoints::searcher::{search_all, search_target};
use crate::endpoints::similarities::{search_similar_docs, search_similar_docs_target};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, Scope};
use dotenv::dotenv;

use derive_builder::Builder;
use std::env::var;
use std::str::FromStr;

#[derive(Builder)]
pub struct ServiceParameters {
    es_host: String,
    es_user: String,
    es_passwd: String,
    service_addr: String,
    service_port: u16,
    cors_origin: String,
}

impl ServiceParameters {
    pub fn es_host(&self) -> &str {
        self.es_host.as_str()
    }

    pub fn es_user(&self) -> &str {
        self.es_user.as_str()
    }

    pub fn es_passwd(&self) -> &str {
        self.es_passwd.as_str()
    }

    pub fn service_address(&self) -> &str {
        self.service_addr.as_str()
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
    }

    pub fn cors_origin(&self) -> String {
        self.cors_origin.clone()
    }
}

pub fn init_service_parameters() -> Result<ServiceParameters, anyhow::Error> {
    dotenv().ok();
    build_env_logger();

    let es_host = var("ELASTIC_HOST").expect("There is not ELASTIC_HOST env variable!");
    let es_user = var("ELASTIC_USER").expect("There is no ELASTIC_USER env variable!");
    let es_passwd = var("ELASTIC_PASSWORD").expect("There is not ELASTIC_PASSWORD env variable!");
    let client_addr = var("SEARCHER_ADDRESS").expect("There is not SEARCHER_ADDRESS env variable!");
    let client_port = var("SEARCHER_PORT").expect("There is not SEARCHER_PORT env variable!");
    let cors_origins: String = var("CORS_ORIGIN").expect("There is not CORS_ORIGIN env variable!");
    let client_port =
        u16::from_str(client_port.as_str()).expect("Failed while parsing port number.");

    let service = ServiceParametersBuilder::default()
        .es_host(es_host)
        .es_user(es_user)
        .es_passwd(es_passwd)
        .service_addr(client_addr)
        .service_port(client_port)
        .cors_origin(cors_origins)
        .build();

    Ok(service.unwrap())
}

pub fn build_cors_config(_origin: &str) -> Cors {
    let available_methods = vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"];
    let available_headers = vec![header::AUTHORIZATION, header::ACCEPT];

    Cors::default()
        .allowed_header(header::CONTENT_TYPE)
        .allowed_methods(available_methods)
        .allowed_headers(available_headers)
        .allow_any_origin()
        .max_age(3600)
}

pub fn build_env_logger() {
    let env_log = env_logger::Env::new();
    let env_log = env_log.default_filter_or("info");
    env_logger::init_from_env(env_log);
}

pub fn build_service() -> Scope {
    web::scope("/searcher")
        .service(hello)
        .service(new_cluster)
        .service(delete_cluster)
        .service(all_clusters)
        .service(get_cluster)
        .service(new_bucket)
        .service(default_bucket)
        .service(delete_bucket)
        .service(all_buckets)
        .service(get_bucket)
        .service(new_document)
        .service(delete_document)
        .service(update_document)
        .service(get_document)
        .service(search_target)
        .service(search_all)
        .service(search_similar_docs)
        .service(search_similar_docs_target)
        .service(load_file)
        .service(upload_file)
}
