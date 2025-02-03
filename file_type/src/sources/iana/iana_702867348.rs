use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_702867348: FileFormat = FileFormat {
    id: 702_867_348,
    source_type: SourceType::Iana,
    name: "ATRAC-ADVANCED-LOSSLESS",
    extensions: &[],
    media_types: &["audio/ATRAC-ADVANCED-LOSSLESS"],
    internal_signatures: &[],
    related_formats: &[],
};
