use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::editor::{
    utils::{add_if_present, create_basic_text_element, from_element_using_builder},
    ChildOfListElement, ElementConverter, HasElementName, UpdatableElement,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Servers {
    #[serde(default, rename = "server")]
    pub servers: Vec<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder, PartialEq)]
pub struct Server {
    pub id: String,
    #[builder(setter(into, strip_option), default)]
    pub username: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub password: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub private_key: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub passphrase: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub file_permissions: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub directory_permissions: Option<String>,
    // TODO: configuration elements
}

impl Server {
    pub fn username_and_password(&self) -> Option<(&str, &str)> {
        match (&self.username, &self.password) {
            (Some(username), Some(password)) => Some((username.as_str(), password.as_str())),
            _ => None,
        }
    }

    pub fn private_key_and_passphrase(&self) -> Option<(&str, &str)> {
        match (&self.private_key, &self.passphrase) {
            (Some(private_key), Some(passphrase)) => {
                Some((private_key.as_str(), passphrase.as_str()))
            }
            _ => None,
        }
    }
}
impl HasElementName for Server {
    fn element_name() -> &'static str {
        "server"
    }
}
impl ElementConverter for Server {
    from_element_using_builder!(
        ServerBuilder,
        element,
        document,
        "id" => id,
        "username" => username,
        "password" => password,
        "privateKey" => private_key,
        "passphrase" => passphrase,
        "filePermissions" => file_permissions,
        "directoryPermissions" => directory_permissions
    );

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            id,
            username,
            password,
            private_key,
            passphrase,
            file_permissions,
            directory_permissions,
        } = self;
        let mut children = vec![create_basic_text_element(document, "id", id)];

        add_if_present!(document, children, username, "username");
        add_if_present!(document, children, password, "password");
        add_if_present!(document, children, private_key, "privateKey");
        add_if_present!(document, children, passphrase, "passphrase");
        add_if_present!(document, children, file_permissions, "filePermissions");
        add_if_present!(
            document,
            children,
            directory_permissions,
            "directoryPermissions"
        );

        Ok(children)
    }
}

impl ChildOfListElement for Server {
    fn parent_element_name() -> &'static str {
        "servers"
    }
}
impl UpdatableElement for Server {
    /// Will rewrite the entire element with the current element. Because it might be a change from a password to a private key.
    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        element.clear_children(document);
        for child in self.clone().into_children(document)? {
            element.push_child(document, child.into())?;
        }
        Ok(())
    }
}
