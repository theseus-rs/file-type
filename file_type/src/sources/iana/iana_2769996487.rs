use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2769996487: FileFormat = FileFormat {
    id: 2_769_996_487,
    source_type: SourceType::Iana,
    name: "vnd.apple.mpegurl",
    extensions: &[],
    media_types: &["application/vnd.apple.mpegurl"],
    signatures: &[],
    related_formats: &[],
};
