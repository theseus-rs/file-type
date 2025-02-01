use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207516: FileFormat = FileFormat {
    id: 28_207_516,
    puid: "wikidata/28207516",
    name: "Word for DOS screen capture",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
