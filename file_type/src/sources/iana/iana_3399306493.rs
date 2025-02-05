use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3399306493: FileFormat = FileFormat {
    id: 3_399_306_493,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawingml.chart+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawingml.chart+xml"],
    signatures: &[],
    related_formats: &[],
};
