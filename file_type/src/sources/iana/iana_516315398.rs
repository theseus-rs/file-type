use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_516315398: FileFormat = FileFormat {
    id: 516_315_398,
    source_type: SourceType::Iana,
    name: "ivs",
    extensions: &[],
    media_types: &["haptics/ivs"],
    internal_signatures: &[],
    related_formats: &[],
};
