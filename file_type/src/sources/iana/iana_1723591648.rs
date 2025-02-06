use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1723591648: FileFormat = FileFormat {
    id: 1_723_591_648,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"],
    signatures: &[],
    related_formats: &[],
};
