use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3992177239: FileFormat = FileFormat {
    id: 3_992_177_239,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"],
    signatures: &[],
    related_formats: &[],
};
