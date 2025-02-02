use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9814302341999196290: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tao intent module archive",
    extensions: &["tao"],
    media_types: &["application/vnd.tao.intent-module-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
