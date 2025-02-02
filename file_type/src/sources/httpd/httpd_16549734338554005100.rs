use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16549734338554005100: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms asf",
    extensions: &["asf", "asx"],
    media_types: &["video/x-ms-asf"],
    internal_signatures: &[],
    related_formats: &[],
};
