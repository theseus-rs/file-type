use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16430238005925662096: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kahootz",
    extensions: &["ktz", "ktr"],
    media_types: &["application/vnd.kahootz"],
    internal_signatures: &[],
    related_formats: &[],
};
