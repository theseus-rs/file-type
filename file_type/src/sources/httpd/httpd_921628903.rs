use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_921628903: FileFormat = FileFormat {
    id: 921_628_903,
    source_type: SourceType::Httpd,
    name: "ms works",
    extensions: &["wps", "wks", "wcm", "wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
