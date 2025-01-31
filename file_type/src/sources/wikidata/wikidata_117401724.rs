use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117401724: FileFormat = FileFormat {
    id: 117_401_724,
    puid: "wikidata/117401724",
    name: "NBT",
    extensions: &["dat", "nbt"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
