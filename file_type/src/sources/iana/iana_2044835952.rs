use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2044835952: FileFormat = FileFormat {
    id: 2_044_835_952,
    source_type: SourceType::Iana,
    name: "raptorfec",
    extensions: &[],
    media_types: &["audio/raptorfec"],
    internal_signatures: &[],
    related_formats: &[],
};
