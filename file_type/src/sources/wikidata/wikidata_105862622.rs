use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862622: FileFormat = FileFormat {
    id: 105_862_622,
    source_type: SourceType::Wikidata,
    name: "Segment Map",
    extensions: &["mps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
