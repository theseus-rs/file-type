use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113557545: FileFormat = FileFormat {
    id: 113_557_545,
    source_type: SourceType::Wikidata,
    name: "Gear Image",
    extensions: &["p01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
