use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2091601761: FileFormat = FileFormat {
    id: 2_091_601_761,
    source_type: SourceType::Iana,
    name: "3mf",
    extensions: &[],
    media_types: &["model/3mf"],
    signatures: &[],
    related_formats: &[],
};
