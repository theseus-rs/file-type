use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_73971162: FileFormat = FileFormat {
    id: 73_971_162,
    source_type: SourceType::Iana,
    name: "provenance-notation",
    extensions: &[],
    media_types: &["text/provenance-notation"],
    internal_signatures: &[],
    related_formats: &[],
};
