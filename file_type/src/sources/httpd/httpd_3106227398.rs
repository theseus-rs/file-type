use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3106227398: FileFormat = FileFormat {
    id: 3_106_227_398,
    source_type: SourceType::Httpd,
    name: "stardivision impress",
    extensions: &["sdd"],
    media_types: &["application/vnd.stardivision.impress"],
    internal_signatures: &[],
    related_formats: &[],
};
