use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445589: FileFormat = FileFormat {
    id: 28_445_589,
    puid: "wikidata/28445589",
    name: "AMOS AmBs",
    extensions: &["abk", "abs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
