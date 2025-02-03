use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_440186036: FileFormat = FileFormat {
    id: 440_186_036,
    source_type: SourceType::Httpd,
    name: "wap wbmp",
    extensions: &["wbmp"],
    media_types: &["image/vnd.wap.wbmp"],
    internal_signatures: &[],
    related_formats: &[],
};
