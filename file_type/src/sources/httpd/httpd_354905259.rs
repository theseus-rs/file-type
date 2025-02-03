use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_354905259: FileFormat = FileFormat {
    id: 354_905_259,
    source_type: SourceType::Httpd,
    name: "sbml xml",
    extensions: &["sbml"],
    media_types: &["application/sbml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
