use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_68397272: FileFormat = FileFormat {
    id: 68_397_272,
    source_type: SourceType::Httpd,
    name: "fastbidsheet",
    extensions: &["fbs"],
    media_types: &["image/vnd.fastbidsheet"],
    signatures: &[],
    related_formats: &[],
};
