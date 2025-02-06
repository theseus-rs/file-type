use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3767960255: FileFormat = FileFormat {
    id: 3_767_960_255,
    source_type: SourceType::Iana,
    name: "vnd.ms-excel",
    extensions: &[],
    media_types: &["application/vnd.ms-excel"],
    signatures: &[],
    related_formats: &[],
};
