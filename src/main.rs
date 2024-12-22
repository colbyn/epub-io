use std::path::PathBuf;
use clap::Parser;
use itertools::Itertools;

pub mod util;
pub mod format;

fn main() {
    match Command::parse() {
        Command::Dump(DumpCommand { input, output, html2text, text_width }) => {
            if !output.exists() {
                std::fs::create_dir_all(&output).expect("Failed to create output directory");
            }
            let mut epub = ::epub::doc::EpubDoc::new(&input).expect("Failed to load input epub file");
            let manifest = crate::format::manifest::Manifest::provision_for(&epub);
            let all_paths = manifest.all_file_paths();
            let exports = epub.resources.clone()
                .into_iter()
                .map(|(resource_id, (path, mime))| (resource_id, path, mime))
                .filter(|(_, _, mime)| crate::util::mime_type::is_content_mime_type(mime))
                .map(|(resource_id, path, _)| (resource_id, path))
                .collect_vec();
            for (resource_id, rel_path) in exports {
                let (file_data, mime_type) = epub.get_resource(&resource_id).expect("Failed to unpack epub content (file entry)");
                assert!(rel_path.is_relative());
                let save = |out_path: PathBuf, file_data: &[u8]| {
                    if let Some(parent) = out_path.parent() {
                        std::fs::create_dir_all(parent).expect("File export failed to provision parent directory(s)");
                    }
                    std::fs::write(out_path, file_data).expect("Failed to write exported file");
                };
                save(output.join(&rel_path), &file_data);
                if crate::util::mime_type::is_html_mime_type(&mime_type) && html2text {
                    let txt_rel_path = rel_path.with_extension("txt");
                    assert!(!all_paths.contains(&txt_rel_path), "internal sanity check");
                    let output_file_path = output.join(txt_rel_path);
                    let reader = std::io::Cursor::new(&file_data);
                    let text_source = html2text::from_read(reader, text_width).expect("Failed to convert html to text");
                    save(output_file_path, text_source.as_bytes());
                }
            }
            manifest.unsafe_write(output.join("manifest.toml"));
        }
    }
}

#[derive(clap::Parser, Debug)]
#[command(name = "epub-io")]
#[command(about = "A CLI tool for dumping the contents of EPUB files and processing content into clean plain text files.", long_about = None)]
enum Command {
    /// Dump the contents of the given input EPUB file to the specified output directory. 
    Dump(DumpCommand),
}

#[derive(clap::Parser, Debug)]
struct DumpCommand {
    /// The file path of the input EPUB file. 
    #[arg(long)]
    input: PathBuf,
    /// The output directory path. 
    /// 
    /// Will create the output directory if missing. 
    #[arg(long)]
    output: PathBuf,
    /// Include plain text versions of the unpacked HTML files using `html2text`.
    #[arg(long)]
    html2text: bool,
    /// When the `html2text` flag is on, the text will be wrapped to `width` columns.
    #[arg(long, default_value_t=80)]
    text_width: usize,
}
