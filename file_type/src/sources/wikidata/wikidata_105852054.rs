use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852054: FileFormat = FileFormat {
    id: 105_852_054,
    puid: "wikidata/105852054",
    name: "atari++ state",
    extensions: &["state"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
