use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_403771798: FileFormat = FileFormat {
    id: 403_771_798,
    source_type: SourceType::Iana,
    name: "oblivious-dns-message",
    extensions: &[],
    media_types: &["application/oblivious-dns-message"],
    internal_signatures: &[],
    related_formats: &[],
};
