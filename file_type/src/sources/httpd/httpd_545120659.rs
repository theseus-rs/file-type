use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_545120659: FileFormat = FileFormat {
    id: 545_120_659,
    source_type: SourceType::Httpd,
    name: "dts",
    extensions: &["dts"],
    media_types: &["audio/vnd.dts"],
    internal_signatures: &[],
    related_formats: &[],
};
