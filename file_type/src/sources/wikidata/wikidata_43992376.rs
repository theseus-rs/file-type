use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43992376: FileType = FileType {
    file_format: &FileFormat {
        id: 43_992_376,
        source_type: SourceType::Wikidata,
        name: "ABIF file format",
        extensions: &["abif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
