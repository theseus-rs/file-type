use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_562569549: FileFormat = FileFormat {
    id: 562_569_549,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"],
    signatures: &[],
    related_formats: &[],
};
