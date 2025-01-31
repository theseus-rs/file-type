use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121914169: FileFormat = FileFormat {
    id: 121_914_169,
    puid: "wikidata/121914169",
    name: "Memory Stick Voice File /Digital Voice File LPEC Codec",
    extensions: &["dvf", "msv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
