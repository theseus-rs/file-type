use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5216488401283368545: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msaccess",
    extensions: &["mdb"],
    media_types: &["application/x-msaccess"],
    internal_signatures: &[],
    related_formats: &[],
};
