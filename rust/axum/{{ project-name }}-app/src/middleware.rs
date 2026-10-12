//! Tower middleware layers.

use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};

/// CORS middleware allowing any origin (adjust for production).
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

/// Response compression middleware (gzip).
pub fn compression_layer() -> CompressionLayer {
    CompressionLayer::new().gzip(true)
}
