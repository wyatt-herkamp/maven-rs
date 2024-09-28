use crate::meta::DeployMetadata;
use crate::meta::SnapshotMetadata;
use crate::pom::editor::PomEditor;
use crate::pom::Pom;
use crate::resolver::ResolvableRepository;
use crate::utils::group_id_and_artifact_id_to_path;
use bytes::Buf;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use thiserror::Error;
use tracing::debug;
use tracing::instrument;

use super::Dependency;
use crate::editor::XMLEditorError;
#[derive(Debug, Error)]
pub enum DependencyResolverError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error("Invalid URL: {0}")]
    InvalidURL(#[from] url::ParseError),
    #[error("Failed to parse metadata")]
    InvalidXMLForQuickXML(#[from] quick_xml::de::DeError),
    #[error("Invalid XML file")]
    InvalidXMLForEditor(#[from] XMLEditorError),
}
impl Dependency {
    #[instrument]
    pub async fn resolve_deploy_meta_data<R: ResolvableRepository>(
        &self,
        repository: R,
        client: &Client,
    ) -> Result<Option<DeployMetadata>, DependencyResolverError> {
        let path = format!(
            "{}/{}",
            group_id_and_artifact_id_to_path(&self.group_id, &self.artifact_id),
            "maven-metadata.xml"
        );
        let url = repository.create_url_with_path(&path)?;
        let response = client.get(url).send().await?;
        if response.status().is_success() {
            let body = response.bytes().await?.reader();
            let metadata = quick_xml::de::from_reader(body)?;
            return Ok(Some(metadata));
        }
        Ok(None)
    }
    #[instrument]
    pub async fn resolve_pom<R: ResolvableRepository>(
        &self,
        repository: R,
        client: &Client,
    ) -> Result<Option<Pom>, DependencyResolverError> {
        let path = self.pom_path();
        let url = repository.create_url_with_path(&path)?;
        debug!(?url, "Resolving POM");

        let response = client.get(url).send().await?;
        if response.status().is_success() {
            let body = response.bytes().await?.reader();
            let pom = quick_xml::de::from_reader(body)?;
            return Ok(Some(pom));
        }
        Ok(None)
    }
    #[instrument]
    pub async fn resolve_pom_as_editor<R: ResolvableRepository>(
        &self,
        repository: R,
        client: &Client,
    ) -> Result<Option<PomEditor>, DependencyResolverError> {
        let path = self.pom_path();
        let url = repository.create_url_with_path(&path)?;
        debug!(?url, "Resolving POM");

        let response = client.get(url).send().await?;
        if response.status().is_success() {
            let body = response.bytes().await?.reader();
            let pom = PomEditor::load_from_reader(body)?;
            return Ok(Some(pom));
        }
        Ok(None)
    }

    #[instrument]
    pub async fn resolve_snapshot_meta<R: ResolvableRepository>(
        &self,
        repository: R,
        client: &Client,
    ) -> Result<Option<SnapshotMetadata>, DependencyResolverError> {
        let path = group_id_and_artifact_id_to_path(&self.group_id, &self.artifact_id);
        let url = repository.create_url_with_path(&path)?;

        let response = client.get(url).send().await?;
        if response.status().is_success() {
            let body = response.bytes().await?.reader();
            let metadata = quick_xml::de::from_reader(body)?;
            return Ok(Some(metadata));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn junit_sunfire_provider() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let dep = super::Dependency {
            group_id: "org.junit.platform".to_string(),
            artifact_id: "junit-platform-surefire-provider".to_string(),
            version: Some("5.8.0".parse().unwrap()),
            depend_type: None,
            scope: None,
            classifier: None,
        };
        let metadata = dep
            .resolve_deploy_meta_data("https://repo1.maven.org/maven2/", &client)
            .await?;
        println!("{:#?}", metadata);
        Ok(())
    }
}
