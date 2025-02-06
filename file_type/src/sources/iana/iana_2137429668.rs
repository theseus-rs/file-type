use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2137429668: FileFormat = FileFormat {
    id: 2_137_429_668,
    source_type: SourceType::Iana,
    name: "vnd.wmf.bootstrap",
    extensions: &[],
    media_types: &["application/vnd.wmf.bootstrap"],
    signatures: &[],
    related_formats: &[],
};
