use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967102: FileFormat = FileFormat {
    id: 27_967_102,
    puid: "wikidata/27967102",
    name: "Nintendo GameCube/Wii AST",
    extensions: &["ast"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
