use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1182453069: FileFormat = FileFormat {
    id: 1_182_453_069,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.pinapp-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.pinapp-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
