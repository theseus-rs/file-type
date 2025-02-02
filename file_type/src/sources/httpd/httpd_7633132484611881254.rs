use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7633132484611881254: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sbml xml",
    extensions: &["sbml"],
    media_types: &["application/sbml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
