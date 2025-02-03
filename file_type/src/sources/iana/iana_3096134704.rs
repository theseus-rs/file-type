use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3096134704: FileFormat = FileFormat {
    id: 3_096_134_704,
    source_type: SourceType::Iana,
    name: "vtt",
    extensions: &[],
    media_types: &["text/vtt"],
    internal_signatures: &[],
    related_formats: &[],
};
