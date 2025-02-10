use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28449455: FileType = FileType {
    file_format: &FileFormat {
        id: 28_449_455,
        source_type: SourceType::Wikidata,
        name: "TOML",
        extensions: &["toml"],
        media_types: &["application/toml"],
        signatures: &[],
        related_formats: &[],
    },
};
