use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1080843780: FileFormat = FileFormat {
    id: 1_080_843_780,
    source_type: SourceType::Iana,
    name: "vnd.wolfram.mathematica",
    extensions: &[],
    media_types: &["application/vnd.wolfram.mathematica"],
    internal_signatures: &[],
    related_formats: &[],
};
