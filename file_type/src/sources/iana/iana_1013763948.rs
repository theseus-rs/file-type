use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1013763948: FileFormat = FileFormat {
    id: 1_013_763_948,
    source_type: SourceType::Iana,
    name: "vnd.openblox.game+xml",
    extensions: &[],
    media_types: &["application/vnd.openblox.game+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
