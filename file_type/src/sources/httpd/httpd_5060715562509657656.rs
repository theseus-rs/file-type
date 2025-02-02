use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5060715562509657656: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pawaafile",
    extensions: &["paw"],
    media_types: &["application/vnd.pawaafile"],
    internal_signatures: &[],
    related_formats: &[],
};
