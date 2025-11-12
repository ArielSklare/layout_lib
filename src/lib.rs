//! layout_lib: cross-platform keyboard layout utilities.
//!
//! This crate exposes:
//! - Getting/replacing highlighted text: `get_highlighted_text`, `replace_highlighted_text`
//! - Querying keyboard layouts and mappings
//! - Inferring a layout for text and shifting text between layouts
//!
//! Functions are re-exported at the crate root for convenience.
//! Platform-specific implementations are selected at compile-time via cfg.

pub mod get_highlighted;
pub mod keyboard_mapping;

// Flattened re-exports at crate root
pub use get_highlighted::{get_highlighted_text, replace_highlighted_text};
pub use keyboard_mapping::{
    all_layout_vk_maps, get_layout, get_text_leyaout_map, list_layouts, shift_text_language,
    vk_to_char_map_default, vk_to_char_map_for_layout, KeyboardDirection, KeyboardLayout, LayoutMap,
};
