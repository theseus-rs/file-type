use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2319413121: FileFormat = FileFormat {
    id: 2_319_413_121,
    source_type: SourceType::Iana,
    name: "parityfec",
    extensions: &[],
    media_types: &["text/parityfec"],
    internal_signatures: &[],
    related_formats: &[],
};
