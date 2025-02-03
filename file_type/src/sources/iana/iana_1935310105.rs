use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1935310105: FileFormat = FileFormat {
    id: 1_935_310_105,
    source_type: SourceType::Iana,
    name: "vnd.etsi.aoc+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.aoc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
