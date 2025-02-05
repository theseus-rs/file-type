use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1610331905: FileFormat = FileFormat {
    id: 1_610_331_905,
    source_type: SourceType::Iana,
    name: "vnd.valve.source.material",
    extensions: &[],
    media_types: &["application/vnd.valve.source.material"],
    signatures: &[],
    related_formats: &[],
};
