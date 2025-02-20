use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116331429: FileType = FileType {
    file_format: &FileFormat {
        id: 116_331_429,
        source_type: SourceType::Wikidata,
        name: "Lawyer File",
        extensions: &["flp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
