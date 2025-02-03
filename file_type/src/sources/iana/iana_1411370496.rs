use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1411370496: FileFormat = FileFormat {
    id: 1_411_370_496,
    source_type: SourceType::Iana,
    name: "SMV",
    extensions: &[],
    media_types: &["audio/SMV"],
    internal_signatures: &[],
    related_formats: &[],
};
