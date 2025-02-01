use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116250851: FileFormat = FileFormat {
    id: 116_250_851,
    puid: "wikidata/116250851",
    name: "CodeWarrior Project file",
    extensions: &["mcp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
