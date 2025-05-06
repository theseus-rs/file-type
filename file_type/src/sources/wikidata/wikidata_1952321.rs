use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1952321: FileType = FileType {
    file_format: &FileFormat {
        id: 1_952_321,
        source_type: SourceType::Wikidata,
        name: "Multi Picture Object",
        extensions: &["jpg", "mpo"],
        media_types: &["image/x-mpo"],
        signatures: &[],
        related_formats: &[],
    },
};
