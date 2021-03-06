use nu_protocol::{Primitive, UntaggedValue, Value};
use nu_source::{AnchorLocation, Tag};
use std::path::Path;

#[derive(Default)]
pub struct TextView;

impl TextView {
    pub fn new() -> TextView {
        TextView
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn view_text_value(value: &Value) {
    let (mut term_width, _) = term_size::dimensions().unwrap_or_else(|| (20, 20));
    let mut tab_width: u64 = 4;
    let mut colored_output = true;
    let mut true_color = true;
    let mut header = true;
    let mut line_numbers = true;
    let mut grid = true;
    let mut vcs_modification_markers = true;
    let mut snip = true;
    let mut wrapping_mode = bat::WrappingMode::NoWrapping;
    let mut use_italics = true;
    let mut paging_mode = bat::PagingMode::QuitIfOneScreen;
    let mut pager = "less".to_string();
    let mut line_ranges = bat::line_range::LineRanges::all();
    let mut _highlight_range = "0,0";
    let highlight_range_from: u64 = 0;
    let highlight_range_to: u64 = 0;
    let mut theme = "OneHalfDark".to_string();

    if let Ok(config) = nu_cli::data::config::config(Tag::unknown()) {
        if let Some(batvars) = config.get("textview") {
            for (idx, value) in batvars.row_entries() {
                match idx.as_ref() {
                    "term_width" => {
                        term_width = match value.as_u64() {
                            Ok(n) => n as usize,
                            _ => term_width as usize,
                        }
                    }
                    "tab_width" => {
                        tab_width = match value.as_u64() {
                            Ok(n) => n,
                            _ => 4u64,
                        }
                    }
                    "colored_output" => {
                        colored_output = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "true_color" => {
                        true_color = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "header" => {
                        header = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "line_numbers" => {
                        line_numbers = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "grid" => {
                        grid = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "vcs_modification_markers" => {
                        vcs_modification_markers = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "snip" => {
                        snip = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "wrapping_mode" => {
                        wrapping_mode = match value.as_string() {
                            Ok(s) if s.to_lowercase() == "nowrapping" => {
                                bat::WrappingMode::NoWrapping
                            }
                            Ok(s) if s.to_lowercase() == "character" => {
                                bat::WrappingMode::Character
                            }
                            _ => bat::WrappingMode::NoWrapping,
                        }
                    }
                    "use_italics" => {
                        use_italics = match value.as_bool() {
                            Ok(b) => b,
                            _ => true,
                        }
                    }
                    "paging_mode" => {
                        paging_mode = match value.as_string() {
                            Ok(s) if s.to_lowercase() == "always" => bat::PagingMode::Always,
                            Ok(s) if s.to_lowercase() == "never" => bat::PagingMode::Never,
                            Ok(s) if s.to_lowercase() == "quitifonescreen" => {
                                bat::PagingMode::QuitIfOneScreen
                            }
                            _ => bat::PagingMode::QuitIfOneScreen,
                        }
                    }
                    "pager" => {
                        pager = match value.as_string() {
                            Ok(s) => s,
                            _ => "less".to_string(),
                        }
                    }
                    "line_ranges" => line_ranges = bat::line_range::LineRanges::all(), // not real sure what to do with this
                    "highlight_range" => _highlight_range = "0,0", //ignore config value for now
                    "theme" => {
                        theme = match value.as_string() {
                            Ok(s) => s,
                            _ => "OneDarkHalf".to_string(),
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    let value_anchor = value.anchor();
    if let UntaggedValue::Primitive(Primitive::String(ref s)) = &value.value {
        if let Some(source) = value_anchor {
            let file_path: Option<String> = match source {
                AnchorLocation::File(file) => {
                    let path = Path::new(&file);
                    Some(path.to_string_lossy().to_string())
                }
                AnchorLocation::Url(url) => {
                    let url = url::Url::parse(&url);
                    if let Ok(url) = url {
                        if let Some(mut segments) = url.path_segments() {
                            if let Some(file) = segments.next_back() {
                                let path = Path::new(file);
                                Some(path.to_string_lossy().to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                //FIXME: this probably isn't correct
                AnchorLocation::Source(_source) => None,
            };

            match file_path {
                Some(file_path) => {
                    // Let bat do it's thing
                    bat::PrettyPrinter::new()
                        .input_from_bytes_with_name(s.as_bytes(), file_path)
                        .term_width(term_width as usize)
                        .tab_width(Some(tab_width as usize))
                        .colored_output(colored_output)
                        .true_color(true_color)
                        .header(header)
                        .line_numbers(line_numbers)
                        .grid(grid)
                        .vcs_modification_markers(vcs_modification_markers)
                        .snip(snip)
                        .wrapping_mode(wrapping_mode)
                        .use_italics(use_italics)
                        .paging_mode(paging_mode)
                        .pager(&pager)
                        .line_ranges(line_ranges)
                        .highlight_range(highlight_range_from as usize, highlight_range_to as usize)
                        .theme(&theme)
                        .print()
                        .expect("Error with bat PrettyPrint");
                }
                _ => {
                    bat::PrettyPrinter::new()
                        .input_from_bytes(s.as_bytes())
                        .term_width(term_width as usize)
                        .tab_width(Some(tab_width as usize))
                        .colored_output(colored_output)
                        .true_color(true_color)
                        .header(header)
                        .line_numbers(line_numbers)
                        .grid(grid)
                        .vcs_modification_markers(vcs_modification_markers)
                        .snip(snip)
                        .wrapping_mode(wrapping_mode)
                        .use_italics(use_italics)
                        .paging_mode(paging_mode)
                        .pager(&pager)
                        .line_ranges(line_ranges)
                        .highlight_range(highlight_range_from as usize, highlight_range_to as usize)
                        .theme(&theme)
                        .print()
                        .expect("Error with bat PrettyPrint");
                }
            }
        } else {
            bat::PrettyPrinter::new()
                .input_from_bytes(s.as_bytes())
                .term_width(term_width as usize)
                .tab_width(Some(tab_width as usize))
                .colored_output(colored_output)
                .true_color(true_color)
                .header(header)
                .line_numbers(line_numbers)
                .grid(grid)
                .vcs_modification_markers(vcs_modification_markers)
                .snip(snip)
                .wrapping_mode(wrapping_mode)
                .use_italics(use_italics)
                .paging_mode(paging_mode)
                .pager(&pager)
                .line_ranges(line_ranges)
                .highlight_range(highlight_range_from as usize, highlight_range_to as usize)
                .theme(&theme)
                .print()
                .expect("Error with bat PrettyPrint");
        }
    }
}
