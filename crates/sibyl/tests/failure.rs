//! Failure-path coverage (FR-017, SC-006): a provider error must surface as a *retryable*
//! error without corrupting the flow. The frontend half of T071 (Playwright) lives in the
//! frontend package; this is the backend half.

use async_trait::async_trait;
use providers::{
    FakeProvider, ImageBytes, ImageProvider, ImageRequest, ProviderError, ProviderId, TextProvider,
    TextRequest, TextResponse,
};
use sibyl::{GenerateIconStyles, Sibyl};
use storage::FsStore;

/// Text succeeds (echo via the fake), image generation always fails retryably.
struct FlakyImage(FakeProvider);

#[async_trait]
impl TextProvider for FlakyImage {
    async fn complete(&self, req: TextRequest) -> Result<TextResponse, ProviderError> {
        self.0.complete(req).await
    }
    fn id(&self) -> ProviderId {
        ProviderId::new("flaky", "text")
    }
}

#[async_trait]
impl ImageProvider for FlakyImage {
    async fn generate(&self, _req: ImageRequest) -> Result<ImageBytes, ProviderError> {
        Err(ProviderError::unavailable("image backend is down"))
    }
    fn id(&self) -> ProviderId {
        ProviderId::new("flaky", "image")
    }
}

#[tokio::test]
async fn provider_failure_surfaces_as_retryable() {
    let dir = tempfile::tempdir().unwrap();
    let forge = Sibyl::new(
        Box::new(FlakyImage(FakeProvider::new())),
        Box::new(FlakyImage(FakeProvider::new())),
        Box::new(FsStore::new(dir.path())),
    );

    let err = forge
        .generate_icon_styles(GenerateIconStyles {
            session_id: None,
            seed: Some(7),
            style: "s".into(),
            count: 3,
        })
        .await
        .expect_err("generation should fail when the image provider is down");

    assert!(
        err.retryable(),
        "failure must be retryable so the owner can retry: {err}"
    );
}
