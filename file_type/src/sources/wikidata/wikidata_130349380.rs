use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130349380: FileFormat = FileFormat {
    id: 130_349_380,
    puid: "wikidata/130349380",
    name: "Modula-2 source code file",
    extensions: &["def", "mod"],
    media_types: &["text/x-modula2", "text/x-modula2"],
    internal_signatures: &[],
    related_formats: &[],
};
