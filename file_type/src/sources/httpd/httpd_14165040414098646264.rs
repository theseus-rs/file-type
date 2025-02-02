use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14165040414098646264: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "epson msf",
    extensions: &["msf"],
    media_types: &["application/vnd.epson.msf"],
    internal_signatures: &[],
    related_formats: &[],
};
