use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2459910717: FileFormat = FileFormat {
    id: 2_459_910_717,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.vmlDrawing",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.vmlDrawing"],
    internal_signatures: &[],
    related_formats: &[],
};
