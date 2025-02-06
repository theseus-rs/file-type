use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_620699404: FileFormat = FileFormat {
    id: 620_699_404,
    source_type: SourceType::Iana,
    name: "dialog-info+xml",
    extensions: &[],
    media_types: &["application/dialog-info+xml"],
    signatures: &[],
    related_formats: &[],
};
