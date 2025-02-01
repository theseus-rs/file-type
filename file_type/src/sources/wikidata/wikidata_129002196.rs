use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129002196: FileFormat = FileFormat {
    id: 129_002_196,
    puid: "wikidata/129002196",
    name: "EBNF file format",
    extensions: &["ebnf"],
    media_types: &["text/x-ebnf"],
    internal_signatures: &[],
    related_formats: &[],
};
