use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849268: FileFormat = FileFormat {
    id: 105_849_268,
    source_type: SourceType::Wikidata,
    name: "GameMaker Studio Project",
    extensions: &["yyp"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
