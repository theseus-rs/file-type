use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3921585630: FileFormat = FileFormat {
    id: 3_921_585_630,
    source_type: SourceType::Iana,
    name: "vnd.mpegurl",
    extensions: &[],
    media_types: &["video/vnd.mpegurl"],
    signatures: &[],
    related_formats: &[],
};
