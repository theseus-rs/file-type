use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1765471557: FileFormat = FileFormat {
    id: 1_765_471_557,
    source_type: SourceType::Iana,
    name: "aptx",
    extensions: &[],
    media_types: &["audio/aptx"],
    internal_signatures: &[],
    related_formats: &[],
};
