use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34284450: FileFormat = FileFormat {
    id: 34_284_450,
    source_type: SourceType::Wikidata,
    name: "Pawn script",
    extensions: &["p"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
