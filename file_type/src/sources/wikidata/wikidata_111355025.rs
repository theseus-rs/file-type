use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355025: FileFormat = FileFormat {
    id: 111_355_025,
    source_type: SourceType::Wikidata,
    name: "exponential 8-bit format",
    extensions: &["u255law"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
