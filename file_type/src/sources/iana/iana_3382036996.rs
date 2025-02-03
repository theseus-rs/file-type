use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3382036996: FileFormat = FileFormat {
    id: 3_382_036_996,
    source_type: SourceType::Iana,
    name: "vnd.oma.cab-address-book+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.cab-address-book+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
