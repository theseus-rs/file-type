use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951646: FileFormat = FileFormat {
    id: 126_951_646,
    puid: "wikidata/126951646",
    name: "Lex source file",
    extensions: &["l"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
