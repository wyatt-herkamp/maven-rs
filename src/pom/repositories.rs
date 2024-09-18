use std::{fmt::Display, str::FromStr};

use derive_builder::Builder;
use edit_xml::Element;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{
    editor::{
        utils::{
            add_if_present, find_or_create_then_set_text_content, sync_element,
            typed_from_element_using_builder,
        },
        ChildOfListElement, ElementConverter, HasElementName, InvalidValueError, PomValue,
        UpdatableElement,
    },
    utils::serde_utils::serde_via_string_types,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repositories {
    #[serde(rename = "repository")]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    #[builder(setter(into, strip_option), default)]
    pub id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,
    pub url: String,
    #[builder(setter(into, strip_option), default)]
    pub layout: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub update_policy: Option<UpdatePolicy>,
    #[builder(setter(into, strip_option), default)]
    pub checksum_policy: Option<ChecksumPolicy>,
    #[builder(setter(into, strip_option), default)]
    pub releases: Option<SubRepositoryRules>,
    #[builder(setter(into, strip_option), default)]
    pub snapshots: Option<SubRepositoryRules>,
}
impl HasElementName for Repository {
    fn element_name() -> &'static str {
        "repository"
    }
}
impl ChildOfListElement for Repository {
    fn parent_element_name() -> &'static str {
        "repositories"
    }
}
impl UpdatableElement for Repository {
    fn is_same_item(&self, other: &Self) -> bool {
        if self.name.is_none() {
            return false;
        }
        self.name == other.name
    }
    fn update_element(
        &self,
        element: Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        sync_element(document, element, "id", self.id.as_deref());
        sync_element(document, element, "name", self.name.as_deref());
        find_or_create_then_set_text_content(document, element, "url", self.url.as_str());
        // TODO: Layout
        Ok(())
    }
}
impl ElementConverter for Repository {
    fn from_element(
        element: edit_xml::Element,
        document: &edit_xml::Document,
    ) -> Result<Self, crate::editor::XMLEditorError> {
        let mut builder = RepositoryBuilder::default();
        for child in element.child_elements(document) {
            match child.name(document) {
                "id" => {
                    builder.id(String::from_element(child, document)?);
                }
                "name" => {
                    builder.name(String::from_element(child, document)?);
                }
                "url" => {
                    builder.url(String::from_element(child, document)?);
                }
                "layout" => {
                    builder.layout(String::from_element(child, document)?);
                }
                "updatePolicy" => {
                    builder.update_policy(UpdatePolicy::from_element(child, document)?);
                }
                "checksumPolicy" => {
                    builder.checksum_policy(ChecksumPolicy::from_element(child, document)?);
                }
                "releases" => {
                    builder.releases(SubRepositoryRules::from_element(child, document)?);
                }
                "snapshots" => {
                    builder.snapshots(SubRepositoryRules::from_element(child, document)?);
                }
                _ => {}
            }
        }
        let result = builder.build()?;
        Ok(result)
    }
    // TODO: Releases, Snapshots

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            id,
            name,
            url,
            layout,
            update_policy,
            checksum_policy,
            releases,
            snapshots,
        } = self;
        let mut children = vec![];
        add_if_present!(document, children, id, "id");
        add_if_present!(document, children, name, "name");
        children.push(crate::editor::utils::create_basic_text_element(
            document, "url", url,
        ));
        add_if_present!(document, children, layout, "layout");
        add_if_present!(document, children, update_policy, "updatePolicy");
        add_if_present!(document, children, checksum_policy, "checksumPolicy");
        if let Some(releases) = releases {
            let element = Element::new(document, "releases");
            let release_children = releases.into_children(document)?;
            for child in release_children {
                element.push_child(document, child.into())?;
            }
            children.push(element);
        }
        if let Some(snapshots) = snapshots {
            let element = Element::new(document, "snapshots");
            let snapshot_children = snapshots.into_children(document)?;
            for child in snapshot_children {
                element.push_child(document, child.into())?;
            }
            children.push(element);
        }
        Ok(children)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SubRepositoryRules {
    #[builder(setter(into, strip_option), default)]
    pub enabled: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub update_policy: Option<UpdatePolicy>,
    #[builder(setter(into, strip_option), default)]
    pub checksum_policy: Option<ChecksumPolicy>,
}
impl ElementConverter for SubRepositoryRules {
    typed_from_element_using_builder!(
        SubRepositoryRulesBuilder,
        element,
        document,
        "enabled"(bool) => enabled,
        "updatePolicy"(UpdatePolicy) => update_policy,
        "checksumPolicy"(ChecksumPolicy) => checksum_policy
    );
    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            enabled,
            update_policy,
            checksum_policy,
        } = self;
        let mut children = vec![];
        add_if_present!(document, children, enabled, "enabled");
        add_if_present!(document, children, update_policy, "updatePolicy");
        add_if_present!(document, children, checksum_policy, "checksumPolicy");

        Ok(children)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "camelCase")]
pub enum ChecksumPolicy {
    Ignore,
    Fail,
    Warn,
}
serde_via_string_types!(ChecksumPolicy);
impl PomValue for ChecksumPolicy {
    fn from_str_for_editor(value: &str) -> Result<Self, InvalidValueError> {
        match value {
            "ignore" => Ok(ChecksumPolicy::Ignore),
            "fail" => Ok(ChecksumPolicy::Fail),
            "warn" => Ok(ChecksumPolicy::Warn),
            _ => Err(InvalidValueError::InvalidValue {
                expected: "ignore, fail, or warn",
                found: value.to_owned(),
            }),
        }
    }
    fn to_string_for_editor(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "camelCase")]
pub enum RepositoryLayout {
    Default,
    Legacy,
}
serde_via_string_types!(RepositoryLayout);
impl PomValue for RepositoryLayout {
    fn from_str_for_editor(value: &str) -> Result<Self, InvalidValueError> {
        match value {
            "default" => Ok(RepositoryLayout::Default),
            "legacy" => Ok(RepositoryLayout::Legacy),
            _ => Err(InvalidValueError::InvalidValue {
                expected: "default or legacy",
                found: value.to_owned(),
            }),
        }
    }
    fn to_string_for_editor(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdatePolicy {
    Always,
    Daily,
    Interval(usize),
    Never,
}
impl FromStr for UpdatePolicy {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(UpdatePolicy::Always),
            "daily" => Ok(UpdatePolicy::Daily),
            "never" => Ok(UpdatePolicy::Never),
            other => {
                if other.starts_with("interval:") {
                    let interval = other.strip_prefix("interval:").ok_or_else(|| {
                        InvalidValueError::InvalidValue {
                            expected: "interval:<number>",
                            found: other.to_owned(),
                        }
                    })?;
                    let interval: usize =
                        interval
                            .parse()
                            .map_err(|_| InvalidValueError::InvalidFormattedValue {
                                error: interval.to_string(),
                            })?;
                    Ok(UpdatePolicy::Interval(interval))
                } else {
                    Err(InvalidValueError::InvalidValue {
                        expected: "always, daily, never, or interval:<number>",
                        found: other.to_owned(),
                    })
                }
            }
        }
    }
}
impl Display for UpdatePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdatePolicy::Always => write!(f, "always"),
            UpdatePolicy::Daily => write!(f, "daily"),
            UpdatePolicy::Interval(interval) => write!(f, "interval:{}", interval),
            UpdatePolicy::Never => write!(f, "never"),
        }
    }
}

impl PomValue for UpdatePolicy {
    fn from_str_for_editor(value: &str) -> Result<Self, InvalidValueError> {
        value.parse()
    }

    fn to_string_for_editor(&self) -> String {
        self.to_string()
    }
}
serde_via_string_types!(UpdatePolicy);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::editor::utils::test_utils;

    use super::*;
    fn inner_layout_test(layout: RepositoryLayout, expected: &str) {
        assert_eq!(layout.to_string(), expected);
        assert_eq!(RepositoryLayout::from_str(expected).unwrap(), layout);
    }
    #[test]
    fn layout() {
        inner_layout_test(RepositoryLayout::Default, "default");
        inner_layout_test(RepositoryLayout::Legacy, "legacy");
    }

    fn inner_update_policy_test(policy: UpdatePolicy, expected: &str) {
        assert_eq!(policy.to_string(), expected);
        assert_eq!(UpdatePolicy::from_str(expected).unwrap(), policy);
    }
    #[test]
    fn update_policy() {
        inner_update_policy_test(UpdatePolicy::Always, "always");
        inner_update_policy_test(UpdatePolicy::Daily, "daily");
        inner_update_policy_test(UpdatePolicy::Interval(5), "interval:5");
        inner_update_policy_test(UpdatePolicy::Never, "never");
    }
    fn inner_checksum_policy(policy: ChecksumPolicy, expected: &str) {
        assert_eq!(policy.to_string(), expected);
        assert_eq!(ChecksumPolicy::from_str(expected).unwrap(), policy);
    }
    #[test]
    fn checksum_policy() {
        inner_checksum_policy(ChecksumPolicy::Ignore, "ignore");
        inner_checksum_policy(ChecksumPolicy::Fail, "fail");
        inner_checksum_policy(ChecksumPolicy::Warn, "warn");
    }

    fn test_parse_methods(value: &str, expected: Repository) -> anyhow::Result<()> {
        let dep_via_edit_xml = test_utils::create_xml_to_element::<Repository>(value)?;
        let dep_via_serde: Repository = quick_xml::de::from_str(value)?;

        assert_eq!(dep_via_edit_xml, expected);
        assert_eq!(dep_via_serde, expected);
        println!("{:#?}", dep_via_edit_xml);

        let dep_serialize_serde = quick_xml::se::to_string(&expected)?;
        println!("Serialized Over Serde \n {}", dep_serialize_serde);
        Ok(())
    }

    #[test]
    fn basic_repository() -> anyhow::Result<()> {
        test_parse_methods(
            r#"
            <repository>
                <id>central</id>
                <name>Maven Central</name>
                <url>https://repo.maven.apache.org/maven2/</url>
            </repository>
        "#,
            Repository {
                id: Some("central".to_string()),
                name: Some("Maven Central".to_string()),
                url: "https://repo.maven.apache.org/maven2/".to_string(),
                ..Default::default()
            },
        )
    }
    #[test]
    fn just_url() -> anyhow::Result<()> {
        test_parse_methods(
            r#"
            <repository>
                <url>https://repo.maven.apache.org/maven2/</url>
            </repository>
        "#,
            Repository {
                url: "https://repo.maven.apache.org/maven2/".to_string(),
                ..Default::default()
            },
        )
    }
    #[test]
    fn with_release_settings() -> anyhow::Result<()> {
        test_parse_methods(
            r#"
                <repository>
                    <url>https://repo.maven.apache.org/maven2/</url>
                    <releases>
                        <enabled>true</enabled>
                        <updatePolicy>daily</updatePolicy>
                        <checksumPolicy>fail</checksumPolicy>
                    </releases>
                </repository>
            "#,
            Repository {
                url: "https://repo.maven.apache.org/maven2/".to_string(),
                releases: Some(SubRepositoryRules {
                    enabled: Some(true),
                    update_policy: Some(UpdatePolicy::Daily),
                    checksum_policy: Some(ChecksumPolicy::Fail),
                }),
                ..Default::default()
            },
        )
    }
    #[test]
    fn with_snapshot_settings() -> anyhow::Result<()> {
        test_parse_methods(
            r#"
                <repository>
                    <url>https://repo.maven.apache.org/maven2/</url>
                    <snapshots>
                        <enabled>true</enabled>
                        <updatePolicy>daily</updatePolicy>
                        <checksumPolicy>fail</checksumPolicy>
                    </snapshots>
                </repository>
            "#,
            Repository {
                url: "https://repo.maven.apache.org/maven2/".to_string(),
                snapshots: Some(SubRepositoryRules {
                    enabled: Some(true),
                    update_policy: Some(UpdatePolicy::Daily),
                    checksum_policy: Some(ChecksumPolicy::Fail),
                }),
                ..Default::default()
            },
        )
    }

    #[test]
    fn with_empty_sub_rules() -> anyhow::Result<()> {
        test_parse_methods(
            r#"
                <repository>
                    <url>https://repo.maven.apache.org/maven2/</url>
                    <releases> </releases>
                    <snapshots/>
                </repository>
            "#,
            Repository {
                url: "https://repo.maven.apache.org/maven2/".to_string(),
                releases: Some(SubRepositoryRules::default()),
                snapshots: Some(SubRepositoryRules::default()),
                ..Default::default()
            },
        )
    }
}
