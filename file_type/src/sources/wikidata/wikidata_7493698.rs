use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7493698: FileType = FileType {
    file_format: &FileFormat {
        id: 7_493_698,
        source_type: SourceType::Wikidata,
        name: "Shell Scrap Object File",
        extensions: &["shs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
