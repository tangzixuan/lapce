use floem::parley::style::FontFamily;
use serde::{Deserialize, Serialize};
use structdesc::FieldNames;

#[derive(FieldNames, Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct UIConfig {
    #[field_names(
        desc = "Set the UI font family. If empty, it uses system default."
    )]
    font_family: String,

    #[field_names(desc = "Set the UI base font size")]
    font_size: usize,

    #[field_names(desc = "Set the icon size in the UI")]
    icon_size: usize,

    #[field_names(
        desc = "Set the header height for panel header and editor tab header"
    )]
    header_height: usize,

    #[field_names(desc = "Set the height for status line")]
    status_height: usize,

    #[field_names(desc = "Set the minimum width for editor tab")]
    tab_min_width: usize,

    #[field_names(desc = "Set the width for scroll bar")]
    scroll_width: usize,

    #[field_names(desc = "Controls the width of drop shadow in the UI")]
    drop_shadow_width: usize,

    #[field_names(desc = "Controls the width of the preview editor")]
    preview_editor_width: usize,

    #[field_names(
        desc = "Set the hover font family. If empty, it uses the UI font family"
    )]
    hover_font_family: String,
    #[field_names(desc = "Set the hover font size. If 0, uses the UI font size")]
    hover_font_size: usize,

    #[field_names(desc = "Trim whitespace from search results")]
    trim_search_results_whitespace: bool,

    #[field_names(desc = "Set the line height for list items")]
    list_line_height: usize,
}

impl UIConfig {
    pub fn font_size(&self) -> usize {
        self.font_size.max(6).min(32)
    }

    pub fn font_family(&self) -> Vec<FontFamily> {
        FontFamily::parse_list(&self.font_family).collect()
    }

    pub fn icon_size(&self) -> usize {
        if self.icon_size == 0 {
            self.font_size() + 2
        } else {
            self.icon_size.max(6).min(32)
        }
    }
}
