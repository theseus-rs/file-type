use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3000733719: FileFormat = FileFormat {
    id: 3_000_733_719,
    source_type: SourceType::Iana,
    name: "rfc822",
    extensions: &[],
    media_types: &["message/rfc822"],
    internal_signatures: &[],
    related_formats: &[],
};
