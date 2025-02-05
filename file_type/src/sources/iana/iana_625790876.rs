use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_625790876: FileFormat = FileFormat {
    id: 625_790_876,
    source_type: SourceType::Iana,
    name: "sep+xml",
    extensions: &[],
    media_types: &["application/sep+xml"],
    signatures: &[],
    related_formats: &[],
};
