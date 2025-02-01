use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440583: FileFormat = FileFormat {
    id: 111_440_583,
    puid: "wikidata/111440583",
    name: "Lua source file",
    extensions: &["lua", "lua"],
    media_types: &["application/x-lua", "text/x-lua"],
    internal_signatures: &[],
    related_formats: &[],
};
