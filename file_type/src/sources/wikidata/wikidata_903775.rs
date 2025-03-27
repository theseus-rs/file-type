use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_903775: FileType = FileType {
    file_format: &FileFormat {
        id: 903_775,
        source_type: SourceType::Wikidata,
        name: "Embedded OpenType",
        extensions: &["eot"],
        media_types: &["application/vnd.ms-fontobject"],
        signatures: &[],
        related_formats: &[],
    },
};
