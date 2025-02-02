use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11891847532744855460: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "webturbo",
    extensions: &["wtb"],
    media_types: &["application/vnd.webturbo"],
    internal_signatures: &[],
    related_formats: &[],
};
