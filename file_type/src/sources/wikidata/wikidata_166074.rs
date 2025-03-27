use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_166074: FileType = FileType {
    file_format: &FileFormat {
        id: 166_074,
        source_type: SourceType::Wikidata,
        name: "Extensible HyperText Markup Language",
        extensions: &["htm", "html", "xht", "xhtml", "xml"],
        media_types: &["application/xhtml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
