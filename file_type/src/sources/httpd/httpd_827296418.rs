use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_827296418: FileFormat = FileFormat {
    id: 827_296_418,
    source_type: SourceType::Httpd,
    name: "jxl",
    extensions: &["jxl"],
    media_types: &["image/jxl"],
    internal_signatures: &[],
    related_formats: &[],
};
