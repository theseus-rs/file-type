use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849268: FileFormat = FileFormat {
    id: 105_849_268,
    source_type: SourceType::Wikidata,
    name: "GameMaker Studio Project",
    extensions: &["yyp"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
