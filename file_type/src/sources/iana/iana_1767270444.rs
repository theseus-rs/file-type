use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1767270444: FileFormat = FileFormat {
    id: 1_767_270_444,
    source_type: SourceType::Iana,
    name: "vnd.drive+json",
    extensions: &[],
    media_types: &["application/vnd.drive+json"],
    signatures: &[],
    related_formats: &[],
};
