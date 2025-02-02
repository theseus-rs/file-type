use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6940775733493568946: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "muvee style",
    extensions: &["msty"],
    media_types: &["application/vnd.muvee.style"],
    internal_signatures: &[],
    related_formats: &[],
};
