use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693921: FileFormat = FileFormat {
    id: 128_693_921,
    puid: "wikidata/128693921",
    name: "BQN file",
    extensions: &["bqn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
