use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1022638803: FileFormat = FileFormat {
    id: 1_022_638_803,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
