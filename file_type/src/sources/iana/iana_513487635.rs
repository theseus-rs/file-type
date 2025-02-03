use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_513487635: FileFormat = FileFormat {
    id: 513_487_635,
    source_type: SourceType::Iana,
    name: "n3",
    extensions: &[],
    media_types: &["text/n3"],
    internal_signatures: &[],
    related_formats: &[],
};
