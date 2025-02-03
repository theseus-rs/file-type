use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_659177723: FileFormat = FileFormat {
    id: 659_177_723,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-user-profile-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-user-profile-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
