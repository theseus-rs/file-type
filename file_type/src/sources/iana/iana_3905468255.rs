use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3905468255: FileFormat = FileFormat {
    id: 3_905_468_255,
    source_type: SourceType::Iana,
    name: "vnd.httphone",
    extensions: &[],
    media_types: &["application/vnd.httphone"],
    signatures: &[],
    related_formats: &[],
};
