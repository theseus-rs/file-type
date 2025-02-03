use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2767865714: FileFormat = FileFormat {
    id: 2_767_865_714,
    source_type: SourceType::Iana,
    name: "vnd.hp-jlyt",
    extensions: &[],
    media_types: &["application/vnd.hp-jlyt"],
    internal_signatures: &[],
    related_formats: &[],
};
