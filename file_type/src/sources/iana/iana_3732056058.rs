use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3732056058: FileFormat = FileFormat {
    id: 3_732_056_058,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
