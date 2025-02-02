use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12706596273811258171: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "epson esf",
    extensions: &["esf"],
    media_types: &["application/vnd.epson.esf"],
    internal_signatures: &[],
    related_formats: &[],
};
