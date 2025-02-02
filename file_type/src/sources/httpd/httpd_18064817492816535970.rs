use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18064817492816535970: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "jisp",
    extensions: &["jisp"],
    media_types: &["application/vnd.jisp"],
    internal_signatures: &[],
    related_formats: &[],
};
