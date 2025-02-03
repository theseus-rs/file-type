use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4154079289: FileFormat = FileFormat {
    id: 4_154_079_289,
    source_type: SourceType::Iana,
    name: "clue_info+xml",
    extensions: &[],
    media_types: &["application/clue_info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
