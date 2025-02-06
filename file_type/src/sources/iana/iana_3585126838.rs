use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3585126838: FileFormat = FileFormat {
    id: 3_585_126_838,
    source_type: SourceType::Iana,
    name: "ulpfec",
    extensions: &[],
    media_types: &["text/ulpfec"],
    signatures: &[],
    related_formats: &[],
};
