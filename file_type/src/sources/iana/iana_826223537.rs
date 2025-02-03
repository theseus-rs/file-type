use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_826223537: FileFormat = FileFormat {
    id: 826_223_537,
    source_type: SourceType::Iana,
    name: "vnd.font-fontforge-sfd",
    extensions: &[],
    media_types: &["application/vnd.font-fontforge-sfd"],
    internal_signatures: &[],
    related_formats: &[],
};
