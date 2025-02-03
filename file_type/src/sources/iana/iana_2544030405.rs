use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2544030405: FileFormat = FileFormat {
    id: 2_544_030_405,
    source_type: SourceType::Iana,
    name: "fhirpath",
    extensions: &[],
    media_types: &["text/fhirpath"],
    internal_signatures: &[],
    related_formats: &[],
};
