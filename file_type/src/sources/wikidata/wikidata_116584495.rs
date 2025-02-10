use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116584495: FileType = FileType {
    file_format: &FileFormat {
        id: 116_584_495,
        source_type: SourceType::Wikidata,
        name: "Microsoft Windows Snipping Tool Snip Data",
        extensions: &["snip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
