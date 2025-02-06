use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2780337060: FileFormat = FileFormat {
    id: 2_780_337_060,
    source_type: SourceType::Httpd,
    name: "mfer",
    extensions: &["mwf"],
    media_types: &["application/vnd.mfer"],
    signatures: &[],
    related_formats: &[],
};
