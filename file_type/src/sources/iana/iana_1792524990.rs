use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1792524990: FileFormat = FileFormat {
    id: 1_792_524_990,
    source_type: SourceType::Iana,
    name: "vnd.antix.game-component",
    extensions: &[],
    media_types: &["application/vnd.antix.game-component"],
    internal_signatures: &[],
    related_formats: &[],
};
