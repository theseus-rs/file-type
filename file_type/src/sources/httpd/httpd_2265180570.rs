use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2265180570: FileFormat = FileFormat {
    id: 2_265_180_570,
    source_type: SourceType::Httpd,
    name: "wap wmlc",
    extensions: &["wmlc"],
    media_types: &["application/vnd.wap.wmlc"],
    internal_signatures: &[],
    related_formats: &[],
};
