use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12237965542013894657: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cpio",
    extensions: &["cpio"],
    media_types: &["application/x-cpio"],
    internal_signatures: &[],
    related_formats: &[],
};
