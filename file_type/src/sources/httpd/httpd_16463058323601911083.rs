use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16463058323601911083: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms wmv",
    extensions: &["wmv"],
    media_types: &["video/x-ms-wmv"],
    internal_signatures: &[],
    related_formats: &[],
};
