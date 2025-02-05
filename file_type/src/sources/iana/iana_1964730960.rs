use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1964730960: FileFormat = FileFormat {
    id: 1_964_730_960,
    source_type: SourceType::Iana,
    name: "ATRAC-X",
    extensions: &[],
    media_types: &["audio/ATRAC-X"],
    signatures: &[],
    related_formats: &[],
};
