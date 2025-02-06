use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_141898700: FileFormat = FileFormat {
    id: 141_898_700,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.crs+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.crs+xml"],
    signatures: &[],
    related_formats: &[],
};
