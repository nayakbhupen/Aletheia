pub mod config;
pub mod domain;
pub mod error;
pub mod item;
pub mod modality;
pub mod tensor;

pub use config::BenchmarkConfig;
pub use domain::Domain;
pub use error::{AletheiaError, Result};
pub use item::{BenchmarkItem, QidGrounding, SurfaceMetrics};
pub use modality::Modality;
pub use tensor::{EvaluationReceipt, TensorDiagnostics, TruthTensor};
