use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3884808644: FileFormat = FileFormat {
    id: 3_884_808_644,
    source_type: SourceType::Httpd,
    name: "stardivision draw",
    extensions: &["sda"],
    media_types: &["application/vnd.stardivision.draw"],
    internal_signatures: &[],
    related_formats: &[],
};
