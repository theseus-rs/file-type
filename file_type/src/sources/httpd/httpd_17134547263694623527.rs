use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17134547263694623527: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "epson quickanime",
    extensions: &["qam"],
    media_types: &["application/vnd.epson.quickanime"],
    internal_signatures: &[],
    related_formats: &[],
};
