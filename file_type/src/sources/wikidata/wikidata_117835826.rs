use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117835826: FileFormat = FileFormat {
    id: 117_835_826,
    puid: "wikidata/117835826",
    name: "Fujitsu DexNET file",
    extensions: &["dxn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
