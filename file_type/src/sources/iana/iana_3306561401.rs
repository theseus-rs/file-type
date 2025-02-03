use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3306561401: FileFormat = FileFormat {
    id: 3_306_561_401,
    source_type: SourceType::Iana,
    name: "vnd.tao.intent-module-archive",
    extensions: &[],
    media_types: &["application/vnd.tao.intent-module-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
