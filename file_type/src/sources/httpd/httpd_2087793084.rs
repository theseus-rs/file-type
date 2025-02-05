use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2087793084: FileFormat = FileFormat {
    id: 2_087_793_084,
    source_type: SourceType::Httpd,
    name: "vtu",
    extensions: &["vtu"],
    media_types: &["model/vnd.vtu"],
    signatures: &[],
    related_formats: &[],
};
