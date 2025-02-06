use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3354284595: FileFormat = FileFormat {
    id: 3_354_284_595,
    source_type: SourceType::Iana,
    name: "mpa-robust",
    extensions: &[],
    media_types: &["audio/mpa-robust"],
    signatures: &[],
    related_formats: &[],
};
