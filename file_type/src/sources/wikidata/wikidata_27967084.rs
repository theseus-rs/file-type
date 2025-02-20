use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967084: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_084,
        source_type: SourceType::Wikidata,
        name: "Game Music Creator",
        extensions: &["gmc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
