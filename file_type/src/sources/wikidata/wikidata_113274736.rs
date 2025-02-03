use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113274736: FileFormat = FileFormat {
    id: 113_274_736,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Online Greeting",
    extensions: &["pso"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
