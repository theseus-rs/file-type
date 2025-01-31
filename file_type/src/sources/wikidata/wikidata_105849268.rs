use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849268: FileFormat = FileFormat {
    id: 105_849_268,
    puid: "wikidata/105849268",
    name: "GameMaker Studio Project",
    extensions: &["yyp"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
