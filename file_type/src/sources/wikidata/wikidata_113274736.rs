use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113274736: FileFormat = FileFormat {
    id: 113_274_736,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Online Greeting",
    extensions: &["pso"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
