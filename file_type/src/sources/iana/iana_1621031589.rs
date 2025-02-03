use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1621031589: FileFormat = FileFormat {
    id: 1_621_031_589,
    source_type: SourceType::Iana,
    name: "vnd.dolby.mlp",
    extensions: &[],
    media_types: &["audio/vnd.dolby.mlp"],
    internal_signatures: &[],
    related_formats: &[],
};
