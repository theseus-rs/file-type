use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113401722: FileFormat = FileFormat {
    id: 113_401_722,
    puid: "wikidata/113401722",
    name: "Linux/i386 Binary Executable File ZMAGIC",
    extensions: &["o", "so"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
