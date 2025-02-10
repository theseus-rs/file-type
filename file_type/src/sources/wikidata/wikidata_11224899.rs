use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_11224899: FileType = FileType {
    file_format: &FileFormat {
        id: 11_224_899,
        source_type: SourceType::Wikidata,
        name: "ish",
        extensions: &["ish"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
