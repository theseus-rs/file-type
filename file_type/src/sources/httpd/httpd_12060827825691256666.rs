use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12060827825691256666: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "iccprofile",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
