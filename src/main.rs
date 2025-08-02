use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::sync::Arc;
use std::{env, path::PathBuf};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};
use tracing::{error, info};

pub mod embedding {
    tonic::include_proto!("embedding");
}

use embedding::{
    EmbeddingRequest, EmbeddingResponse, TextBatch, Vector,
    embedding_service_server::{EmbeddingService, EmbeddingServiceServer},
};

pub struct EmbeddingServiceImpl {
    model: Arc<Mutex<TextEmbedding>>,
}

impl EmbeddingServiceImpl {
    pub fn new() -> anyhow::Result<Self> {
        let cache_path = PathBuf::from(".cache");
        if !cache_path.exists() {
            std::fs::create_dir_all(&cache_path)?;
            info!(
                "[Embedding-Service] Created cache directory at {:?}",
                cache_path
            );
        }
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::ParaphraseMLMiniLML12V2Q)
                .with_show_download_progress(true)
                .with_cache_dir(cache_path),
        )?;

        info!("[Embedding-Service] Model loaded successfully");
        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }
}

#[tonic::async_trait]
impl EmbeddingService for EmbeddingServiceImpl {
    async fn get_embeddings(
        &self,
        request: Request<EmbeddingRequest>,
    ) -> Result<Response<EmbeddingResponse>, Status> {
        let req = request.into_inner();

        let texts = match req.input {
            Some(embedding::embedding_request::Input::SingleText(text)) => {
                vec![text]
            }
            Some(embedding::embedding_request::Input::BatchTexts(TextBatch { texts })) => texts,
            None => {
                return Err(Status::invalid_argument("No input provided"));
            }
        };

        if texts.is_empty() {
            return Err(Status::invalid_argument("Empty input provided"));
        }

        info!(
            "[Embedding-Service] Processing text(s): [{}]",
            texts.join(", ")
        );

        let mut model = self.model.lock().await;
        match model.embed(texts, None) {
            Ok(embeddings) => {
                let vectors: Vec<Vector> = embeddings
                    .into_iter()
                    .map(|embedding| Vector { values: embedding })
                    .collect();

                info!("[Embedding-Service] Successfully generate embedding(s)");

                Ok(Response::new(EmbeddingResponse { vectors }))
            }
            Err(e) => {
                error!("[Embedding-Service] Failed to generate embeddings: {}", e);
                Err(Status::internal("Failed to generate embeddings"))
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let addr = env::var("APP_SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:6010".to_string())
        .parse()?;

    info!("[Embedding-Service] Starting embedding service on {}", addr);

    let embedding_service = EmbeddingServiceImpl::new()?;

    Server::builder()
        .add_service(EmbeddingServiceServer::new(embedding_service))
        .serve(addr)
        .await?;

    Ok(())
}
