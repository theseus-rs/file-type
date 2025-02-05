use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3202991632: FileFormat = FileFormat {
    id: 3_202_991_632,
    source_type: SourceType::Iana,
    name: "vnd.balsamiq.bmml+xml",
    extensions: &[],
    media_types: &["application/vnd.balsamiq.bmml+xml"],
    signatures: &[],
    related_formats: &[],
};
