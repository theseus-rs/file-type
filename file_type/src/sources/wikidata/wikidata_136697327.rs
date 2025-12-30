use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136697327: FileType = FileType {
    file_format: &FileFormat {
        id: 136_697_327,
        source_type: SourceType::Wikidata,
        name: "HTTP Cache",
        extensions: &["http"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
