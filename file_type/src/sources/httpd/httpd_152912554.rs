use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_152912554: FileFormat = FileFormat {
    id: 152_912_554,
    source_type: SourceType::Httpd,
    name: "semf",
    extensions: &["semf"],
    media_types: &["application/vnd.semf"],
    signatures: &[],
    related_formats: &[],
};
