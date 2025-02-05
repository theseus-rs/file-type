use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3461058363: FileFormat = FileFormat {
    id: 3_461_058_363,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.presProps+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"],
    signatures: &[],
    related_formats: &[],
};
