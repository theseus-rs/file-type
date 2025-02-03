use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_354905259: FileFormat = FileFormat {
    id: 354_905_259,
    source_type: SourceType::Iana,
    name: "sbml+xml",
    extensions: &[],
    media_types: &["application/sbml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
