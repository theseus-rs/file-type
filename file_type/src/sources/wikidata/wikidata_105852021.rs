use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852021: FileFormat = FileFormat {
    id: 105_852_021,
    puid: "wikidata/105852021",
    name: "SuperCollider Class",
    extensions: &["sc", "sc", "sc"],
    media_types: &[
        "application/supercollider",
        "text/plain",
        "text/supercollider",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
