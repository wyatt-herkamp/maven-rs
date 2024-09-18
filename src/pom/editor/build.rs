use edit_xml::Element;

use crate::{
    editor::utils::{add_or_update_item, find_element, get_all_children_of_element},
    editor::XMLEditorError,
    pom::build::Plugin,
};

use super::PomEditor;
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
    use crate::pom::editor::{build::Plugin, PomEditor};

    #[test]
    pub fn test_plugins() -> anyhow::Result<()> {
        let mut editor = PomEditor::new_with_group_and_artifact("dev.wyatt-herkamp", "test");
        {
            let mut build_editor = editor.build_editor();
            build_editor.set_source_directory("src/main/java");
            build_editor.set_final_name("test");
            let plugin = Plugin {
                group_id: "org.apache.maven.plugins".to_string(),
                artifact_id: "maven-compiler-plugin".to_string(),
                version: "3.8.1".to_string(),
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
