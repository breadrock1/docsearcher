use crate::errors::{SuccessfulResponse, WebResponse};
use crate::searcher::own_engine::context::OtherContext;
use crate::searcher::service_client::ServiceClient;
use crate::wrappers::bucket::{Bucket, BucketForm};
use crate::wrappers::cluster::Cluster;
use crate::wrappers::document::Document;
use crate::wrappers::search_params::SearchParameters;

use actix_web::{web, HttpResponse};

#[async_trait::async_trait]
impl ServiceClient for OtherContext {
    async fn get_all_clusters(&self) -> WebResponse<web::Json<Vec<Cluster>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn get_cluster(&self, _cluster_id: &str) -> WebResponse<web::Json<Cluster>> {
        Ok(web::Json(Cluster::default()))
    }

    async fn create_cluster(&self, _cluster_id: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn delete_cluster(&self, _cluster_id: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn get_all_buckets(&self) -> WebResponse<web::Json<Vec<Bucket>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn get_bucket(&self, _bucket_id: &str) -> WebResponse<web::Json<Bucket>> {
        Ok(web::Json(Bucket::default()))
    }

    async fn delete_bucket(&self, _bucket_id: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn create_bucket(&self, _bucket_form: &BucketForm) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn get_document(
        &self,
        _bucket_id: &str,
        _doc_id: &str,
    ) -> WebResponse<web::Json<Document>> {
        Ok(web::Json(Document::default()))
    }

    async fn create_document(&self, _doc_form: &Document) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn update_document(&self, _doc_form: &Document) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn delete_document(&self, _bucket_id: &str, _doc_id: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn search_from_all(
        &self,
        _s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn search_from_target(
        &self,
        _bucket_id: &str,
        _s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn similar_from_all(
        &self,
        _s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn similar_from_target(
        &self,
        _bucket_id: &str,
        _s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }
}
