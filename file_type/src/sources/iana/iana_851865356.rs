use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_851865356: FileFormat = FileFormat {
    id: 851_865_356,
    source_type: SourceType::Iana,
    name: "vnd.mdl",
    extensions: &[],
    media_types: &["application/vnd.mdl"],
    internal_signatures: &[],
    related_formats: &[],
};
