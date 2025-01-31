use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207252: FileFormat = FileFormat {
    id: 28_207_252,
    puid: "wikidata/28207252",
    name: "SCR",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
