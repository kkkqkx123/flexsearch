use bm25_service::Config;
use bm25_service::proto::{
    BatchIndexDocumentsRequest, BatchIndexDocumentsResponse, DeleteDocumentRequest,
    DeleteDocumentResponse, GetStatsRequest, GetStatsResponse, IndexDocumentRequest,
    IndexDocumentResponse, SearchRequest, SearchResponse,
};
use bm25_service::proto::Bm25Service as Bm25ServiceTrait;
use bm25_service::proto::Bm25ServiceServer;
use tonic::{transport::Server, Request, Response, Status};

pub struct BM25Service {
    _config: Config,
}

impl BM25Service {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }
}

#[tonic::async_trait]
impl Bm25ServiceTrait for BM25Service {
    async fn index_document(
        &self,
        _request: Request<IndexDocumentRequest>,
    ) -> Result<Response<IndexDocumentResponse>, Status> {
        tracing::info!("Received index document request");
        Ok(Response::new(IndexDocumentResponse {
            success: true,
            message: "Document indexed successfully".to_string(),
        }))
    }

    async fn batch_index_documents(
        &self,
        request: Request<BatchIndexDocumentsRequest>,
    ) -> Result<Response<BatchIndexDocumentsResponse>, Status> {
        tracing::info!("Received batch index documents request");
        let req = request.into_inner();
        Ok(Response::new(BatchIndexDocumentsResponse {
            success: true,
            message: "Documents indexed successfully".to_string(),
            indexed_count: req.documents.len() as i32,
        }))
    }

    async fn search(
        &self,
        _request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        tracing::info!("Received search request");
        Ok(Response::new(SearchResponse {
            results: vec![],
            total: 0,
            max_score: 0.0,
        }))
    }

    async fn delete_document(
        &self,
        _request: Request<DeleteDocumentRequest>,
    ) -> Result<Response<DeleteDocumentResponse>, Status> {
        tracing::info!("Received delete document request");
        Ok(Response::new(DeleteDocumentResponse {
            success: true,
            message: "Document deleted successfully".to_string(),
        }))
    }

    async fn get_stats(
        &self,
        _request: Request<GetStatsRequest>,
    ) -> Result<Response<GetStatsResponse>, Status> {
        tracing::info!("Received get stats request");
        Ok(Response::new(GetStatsResponse {
            total_documents: 0,
            total_terms: 0,
            avg_document_length: 0.0,
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bm25_service::init_logging();
    bm25_service::init_metrics();

    tracing::info!("Starting BM25 service");

    let config = Config::from_env().unwrap_or_else(|_| Config::default());
    tracing::info!("Loaded configuration: {:?}", config);

    let addr = config.server.address;
    tracing::info!("BM25 service listening on {}", addr);

    let bm25_service = BM25Service::new(config);

    Server::builder()
        .add_service(Bm25ServiceServer::new(bm25_service))
        .serve(addr)
        .await?;

    Ok(())
}
