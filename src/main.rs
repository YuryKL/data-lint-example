use std::env;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor, Language};
use streaming_iterator::StreamingIterator;

const PRO_PLUGINS: &[&str] = &[
    "animate",
    "custom-validity",
    "on-raf",
    "on-resize",
    "persist",
    "query-string",
    "replace-url",
    "rocket",
    "scroll-into-view",
    "view-transition",
];

struct Warning {
    file: String,
    line: usize,
    column: usize,
    plugin: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: data-lint <file-or-directory>...");
        std::process::exit(1);
    }

    let mut warnings = Vec::new();

    for path in &args[1..] {
        if let Err(e) = process_path(path, &mut warnings) {
            eprintln!("Error processing {}: {}", path, e);
            std::process::exit(1);
        }
    }

    if warnings.is_empty() {
        println!("✓ No pro plugins detected!");
        return;
    }

    for w in &warnings {
        println!(
            "{}:{}:{}: data-{} - You are using a pro feature",
            w.file, w.line, w.column, w.plugin
        );
    }
}

fn process_path(path: &str, warnings: &mut Vec<Warning>) -> std::io::Result<()> {
    let path = Path::new(path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.is_file() && is_html_file(&file_path) {
                lint_file(file_path.to_str().unwrap(), warnings)?;
            }
        }
    } else if is_html_file(path) {
        lint_file(path.to_str().unwrap(), warnings)?;
    }

    Ok(())
}

fn is_html_file(path: &Path) -> bool {
    let filename = path.to_str().unwrap_or("");
    if filename.ends_with(".blade.php") {
        return true;
    }
    if let Some(ext) = path.extension() {
        matches!(ext.to_str(), Some("html") | Some("heex") | Some("templ"))
    } else {
        false
    }
}

fn get_language_for_file(filename: &str) -> Option<Language> {
    if filename.ends_with(".blade.php") {
        return Some(tree_sitter_blade::LANGUAGE.into()) ;
    }
    let ext = Path::new(filename).extension()?.to_str()?;
    match ext {
        "html" => Some( tree_sitter_html::LANGUAGE.into() ),
        "heex" => Some( tree_sitter_heex::LANGUAGE.into() ),
        "templ" => Some( tree_sitter_templ::LANGUAGE.into() ),
        _ => None,
    }
}

fn lint_file(filename: &str, warnings: &mut Vec<Warning>) -> std::io::Result<()> {
    let source = fs::read_to_string(filename)?;

    // Get the appropriate parser for this file type
    let language = match get_language_for_file(filename) {
        Some(lang) => lang,
        None => return Ok(()), // Skip unsupported files
    };

    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let tree = parser.parse(&source, None).unwrap();
    let root_node = tree.root_node();

    // Query for all attribute nodes with data-* names
    let query_str = r#"
        (attribute
          (attribute_name) @attr_name
          (quoted_attribute_value)? @attr_value)
    "#;

    let query = Query::new(&language, query_str).unwrap();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root_node, source.as_bytes());

    // Set up datastar parser
    let mut datastar_parser = Parser::new();
    let datastar_language: Language = tree_sitter_datastar::LANGUAGE.into();
    datastar_parser.set_language(&datastar_language).unwrap();

    while let Some(m) = matches.next() {
        let attr_name_node = m.captures.iter()
            .find(|c| c.index == 0)
            .map(|c| c.node);

        if let Some(attr_node) = attr_name_node {
            let attr_name = &source[attr_node.byte_range()];

            // Check if it's a data-* attribute
            if !attr_name.starts_with("data-") {
                continue;
            }

            // Parse the attribute name with datastar parser
            if let Some(datastar_tree) = datastar_parser.parse(attr_name, None) {
                let plugin_name = find_plugin_name(datastar_tree.root_node(), attr_name);

                if let Some(plugin) = plugin_name {
                    if PRO_PLUGINS.contains(&plugin) {
                        let start_pos = attr_node.start_position();
                        warnings.push(Warning {
                            file: filename.to_string(),
                            line: start_pos.row + 1,
                            column: start_pos.column + 1,
                            plugin: plugin.to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

fn find_plugin_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() == "plugin_name" {
        return Some(&source[node.byte_range()]);
    }

    for child in node.children(&mut node.walk()) {
        if let Some(plugin) = find_plugin_name(child, source) {
            return Some(plugin);
        }
    }

    None
}
