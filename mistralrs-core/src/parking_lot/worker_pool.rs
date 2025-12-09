//! Worker pool wrapper for prometheus-parking-lot.
//!
//! This module provides `InferenceWorkerPool`, which wraps prometheus-parking-lot's
//! `WorkerPool` and integrates it with mistral.rs inference pipeline.

use super::{
    InferenceJob, InferenceResult, LlmExecutor, StreamingRegistry,
    TaskMetadata, TaskExecutor,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

/// Configuration for the inference worker pool.
#[derive(Debug, Clone)]
pub struct InferenceWorkerPoolConfig {
    /// Number of dedicated worker threads (default: num_cpus)
    pub worker_count: usize,

    /// Maximum resource units (GPU VRAM in MB or KV cache blocks)
    pub max_units: u32,

    /// Maximum queue depth before rejection
    pub max_queue_depth: usize,

    /// Default timeout for job execution in seconds
    pub timeout_secs: u64,
}

impl Default for InferenceWorkerPoolConfig {
    fn default() -> Self {
        Self {
            worker_count: num_cpus::get(),
            max_units: 16384, // ~256K tokens with 16-token blocks
            max_queue_depth: 1000,
            timeout_secs: 120,
        }
    }
}

impl InferenceWorkerPoolConfig {
    /// Create a new config with explicit values.
    #[must_use]
    pub fn new(worker_count: usize, max_units: u32, max_queue_depth: usize) -> Self {
        Self {
            worker_count,
            max_units,
            max_queue_depth,
            timeout_secs: 120,
        }
    }

    /// Set the timeout in seconds.
    #[must_use]
    pub fn with_timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }
}

/// Pool statistics for monitoring.
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// Number of active workers
    pub active_workers: usize,
    /// Number of queued tasks
    pub queued_tasks: usize,
    /// Available capacity (resource units)
    pub available_capacity: u32,
    /// Total capacity (resource units)
    pub total_capacity: u32,
}

/// Worker pool for LLM inference using prometheus-parking-lot.
pub struct InferenceWorkerPool {
    /// The LLM executor for processing jobs
    executor: Arc<LlmExecutor>,

    /// Streaming channel registry for non-serializable results
    streaming_registry: Arc<StreamingRegistry>,

    /// Configuration
    config: InferenceWorkerPoolConfig,
}

impl InferenceWorkerPool {
    /// Create a new inference worker pool.
    ///
    /// # Arguments
    ///
    /// * `config` - Pool configuration
    /// * `executor` - The LLM executor for processing jobs
    ///
    /// # Returns
    ///
    /// The worker pool instance or an error if creation fails
    pub fn new(
        config: InferenceWorkerPoolConfig,
        executor: LlmExecutor,
    ) -> Result<Self, String> {
        info!(
            worker_count = config.worker_count,
            max_units = config.max_units,
            max_queue_depth = config.max_queue_depth,
            "Creating inference worker pool"
        );

        let streaming_registry = Arc::new(StreamingRegistry::with_default_retention());

        // Start background cleanup task
        let registry_for_cleanup = streaming_registry.as_ref().clone();
        registry_for_cleanup.start_cleanup_task(Duration::from_secs(300)); // Cleanup every 5 minutes

        Ok(Self {
            executor: Arc::new(executor),
            streaming_registry,
            config,
        })
    }

    /// Submit an inference job to the pool.
    ///
    /// # Arguments
    ///
    /// * `job` - The inference job to execute
    /// * `meta` - Task metadata (priority, cost, etc.)
    ///
    /// # Returns
    ///
    /// A receiver for the inference result
    pub async fn submit(
        &self,
        job: InferenceJob,
        meta: TaskMetadata,
    ) -> Result<InferenceResult, String> {
        info!(
            task_id = %meta.id,
            request_id = %job.request_id,
            "Submitting job to worker pool"
        );

        // Execute the job directly through the executor
        let result = self.executor.as_ref().execute(job, meta).await;
        Ok(result)
    }

    /// Get pool statistics.
    #[must_use]
    pub fn stats(&self) -> PoolStats {
        // TODO: Get actual stats from WorkerPool
        PoolStats {
            active_workers: self.config.worker_count,
            queued_tasks: 0,
            available_capacity: self.config.max_units,
            total_capacity: self.config.max_units,
        }
    }

    /// Shutdown the worker pool gracefully.
    pub async fn shutdown(&self) -> Result<(), String> {
        info!("Shutting down inference worker pool");
        // TODO: Implement graceful shutdown
        Ok(())
    }
}
