use embedding_db::grpc::embedding_db_server::EmbeddingDbServer;
use embedding_db::handler::EmbeddingDBHandler;
use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let embedding_handler = EmbeddingDBHandler::default();

    Server::builder()
        .add_service(EmbeddingDbServer::new(embedding_handler))
        .serve(addr)
        .await?;

    Ok(())
}
