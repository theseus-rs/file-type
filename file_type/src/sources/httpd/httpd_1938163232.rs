use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1938163232: FileFormat = FileFormat {
    id: 1_938_163_232,
    source_type: SourceType::Httpd,
    name: "zul",
    extensions: &["zir", "zirz"],
    media_types: &["application/vnd.zul"],
    internal_signatures: &[],
    related_formats: &[],
};
