use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2721747258: FileFormat = FileFormat {
    id: 2_721_747_258,
    source_type: SourceType::Iana,
    name: "collection",
    extensions: &[],
    media_types: &["font/collection"],
    signatures: &[],
    related_formats: &[],
};
