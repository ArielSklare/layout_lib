## layout_lib

Cross-platform keyboard layout utilities for:
- Getting/replacing highlighted text
- Querying keyboard layouts and VK-to-char maps
- Inferring a layout for text and shifting text between layouts

### Features
- OS-specific implementations selected via `cfg`:
  - Windows: UI Automation for selection; Win32 keyboard APIs
  - Linux: wl-paste/xclip/xsel for selection; xkbcommon for layouts

### Install
Add to your `Cargo.toml`:

```toml
layout_lib = { path = "../layout_lib" }
```

### Usage

```rust
use layout_lib::*;

fn main() {
    // Get highlighted text (platform-specific)
    if let Some(text) = get_highlighted_text() {
        println!("Selected: {text}");
    }

    // List layouts and get key maps
    let layouts = list_layouts();
    println!("Layouts: {:?}", layouts);

    let maps = all_layout_vk_maps();
    if let (Some(first), Some(second)) = (maps.get(0), maps.get(1)) {
        // Infer layout for text and shift it
        let text = "shalom";
        let shifted = shift_text_language(text, first, second);
        println!("Shifted: {shifted}");
    }
}
```

### Linux requirements
- `xkbcommon` (system library)
- One of: `wl-paste` (Wayland), `xclip`/`xsel` (X11)
- Optional typing tools: `wtype` or `xdotool` for replace

### Windows requirements
- Uses `windows` crate COM/UI Automation and keyboard APIs. No extra setup.

### Public API
Re-exported at crate root:

```rust
// Selection
get_highlighted_text();
replace_highlighted_text(text);

// Layouts
list_layouts();
get_layout(index);
vk_to_char_map_default();
vk_to_char_map_for_layout(index_or_hkl);
all_layout_vk_maps();

// Shift utilities
get_text_leyaout_map(text, &maps);
shift_text_language(text, &from, &to);

// Types
KeyboardLayout; KeyboardDirection; LayoutMap;
```

### License
MIT


