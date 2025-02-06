use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3306561401: FileFormat = FileFormat {
    id: 3_306_561_401,
    source_type: SourceType::Httpd,
    name: "tao intent module archive",
    extensions: &["tao"],
    media_types: &["application/vnd.tao.intent-module-archive"],
    signatures: &[],
    related_formats: &[],
};
