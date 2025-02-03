use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2211731750: FileFormat = FileFormat {
    id: 2_211_731_750,
    source_type: SourceType::Iana,
    name: "flexfec",
    extensions: &[],
    media_types: &["audio/flexfec"],
    internal_signatures: &[],
    related_formats: &[],
};
