use edit_xml::Element;

use crate::{
    editor::utils::{add_or_update_item, find_element, get_all_children_of_element},
    editor::XMLEditorError,
    pom::build::Plugin,
};

use super::PomEditor;
impl PomEditor {
    /// Creates a new build editor
    ///
    /// If no build element is present, it will create one
    /// # Note.
    /// This function will hold a mutable reference to the PomEditor.
    /// I would recommend using this function within a scope. To prevent borrowing issues.
    pub fn get_or_create_build_element(&mut self) -> BuildEditor<'_> {
        return BuildEditor::new(self);
    }
    /// Checks if the build element is present in the pom file
    ///
    /// If the build element is present, it will return Some(BuildEditor) else it will return None
    pub fn get_build_element_or_none(&mut self) -> Option<BuildEditor<'_>> {
        if self.has_build() {
            return Some(BuildEditor::new(self));
        }
        return None;
    }
    pub fn has_build(&self) -> bool {
        let root = self.root();
        find_element(root, "build", &self.document).is_some()
    }
    pub fn delete_build(&mut self) -> Result<bool, XMLEditorError> {
        let root = self.root();
        let element = find_element(root, "build", &self.document);
        if let Some(element) = element {
            element.detach(&mut self.document)?;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}
/// Allows for editing the build section of a pom file
#[derive(Debug)]
pub struct BuildEditor<'a> {
    parent: &'a mut PomEditor,
    build_element: Element,
}
macro_rules! top_level_getter_setter {
    (
        $set:ident, $get:ident, $name:literal
    ) => {
        pub fn $set(&mut self, value: &str) {
            let element = crate::editor::utils::get_or_create_top_level_element(
                $name,
                &mut self.parent.document,
                self.build_element,
            );
            element.set_text_content(&mut self.parent.document, value);
        }
        pub fn $get(&self) -> Option<String> {
            let element = crate::editor::utils::find_element(
                self.build_element,
                $name,
                &self.parent.document,
            );
            return element.map(|x| x.text_content(&self.parent.document));
        }
    };
}
impl<'a> BuildEditor<'a> {
    top_level_getter_setter!(
        set_source_directory,
        get_source_directory,
        "sourceDirectory"
    );
    top_level_getter_setter!(set_final_name, get_final_name, "finalName");
    top_level_getter_setter!(set_directory, get_directory, "directory");
    top_level_getter_setter!(set_default_goal, get_default_goal, "defaultGoal");

    pub(super) fn new(parent: &'a mut PomEditor) -> Self {
        let root = parent.root();
        let build_element = crate::editor::utils::get_or_create_top_level_element(
            "build",
            &mut parent.document,
            root,
        );
        Self {
            parent,
            build_element,
        }
    }
    /// Gets all the plugins in the build section
    pub fn get_plugins(&self) -> Result<Vec<Plugin>, XMLEditorError> {
        let Some(plugins) = find_element(self.build_element, "plugins", &self.parent.document)
        else {
            return Ok(vec![]);
        };
        let result = get_all_children_of_element::<Plugin>(&self.parent.document, plugins)?;
        Ok(result.into_iter().map(|(depend, _)| depend).collect())
    }
    /// Adds or updates a plugin in the build section
    pub fn add_or_update_plugin(
        &mut self,
        plugin: Plugin,
    ) -> Result<Option<Plugin>, XMLEditorError> {
        let plugins = find_element(self.build_element, "plugins", &self.parent.document);
        add_or_update_item(
            &mut self.parent.document,
            plugins,
            self.build_element,
            plugin,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pom::editor::{build::Plugin, PomEditor},
        types::Property,
    };

    #[test]
    pub fn test_plugins() -> anyhow::Result<()> {
        let mut editor = PomEditor::new_with_group_and_artifact("dev.wyatt-herkamp", "test");
        {
            let mut build_editor = editor.get_or_create_build_element();
            build_editor.set_source_directory("src/main/java");
            build_editor.set_final_name("test");
            let plugin = Plugin {
                group_id: Some("org.apache.maven.plugins".to_string()),
                artifact_id: "maven-compiler-plugin".to_string(),
                version: Some(Property::Literal("3.8.1".to_string())),
            };
            build_editor.add_or_update_plugin(plugin.clone())?;
            let plugins = build_editor.get_plugins()?;
            assert_eq!(plugins.len(), 1);
            assert_eq!(plugins[0], plugin);
        }
        let value = editor.write_to_str()?;
        println!("{}", value);

        Ok(())
    }
}
