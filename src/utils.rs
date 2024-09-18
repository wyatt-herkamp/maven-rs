pub(crate) mod serde_utils;
pub mod time;
pub fn group_id_to_path(group_id: &str) -> String {
    group_id.replace(".", "/")
}

pub fn group_id_and_artifact_id_to_path(group_id: &str, artifact_id: &str) -> String {
    format!("{}/{}", group_id_to_path(group_id), artifact_id)
}

pub fn group_id_and_artifact_id_and_version_to_path(
    group_id: &str,
    artifact_id: &str,
    version: &str,
) -> String {
    format!("{}/{}/{}", group_id_to_path(group_id), artifact_id, version)
}
