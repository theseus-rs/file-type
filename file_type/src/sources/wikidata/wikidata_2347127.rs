use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2347127: FileType = FileType {
    file_format: &FileFormat {
        id: 2_347_127,
        source_type: SourceType::Wikidata,
        name: "Compressed image format",
        extensions: &["cso"],
        media_types: &["application/x-compressed-iso"],
        signatures: &[],
        related_formats: &[],
    },
};
