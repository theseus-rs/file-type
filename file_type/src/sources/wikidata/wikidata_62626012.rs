use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62626012: FileType = FileType {
    file_format: &FileFormat {
        id: 62_626_012,
        source_type: SourceType::Wikidata,
        name: "HyperText Markup Language",
        extensions: &["htm", "html", "xht", "xhtml"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
