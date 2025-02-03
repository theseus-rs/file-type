use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1108063713: FileFormat = FileFormat {
    id: 1_108_063_713,
    source_type: SourceType::Iana,
    name: "prs.rdf-xml-crypt",
    extensions: &[],
    media_types: &["application/prs.rdf-xml-crypt"],
    internal_signatures: &[],
    related_formats: &[],
};
