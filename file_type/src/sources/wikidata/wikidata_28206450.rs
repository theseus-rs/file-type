use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206450: FileFormat = FileFormat {
    id: 28_206_450,
    source_type: SourceType::Wikidata,
    name: "KiSS CEL 8-bit",
    extensions: &["cel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
