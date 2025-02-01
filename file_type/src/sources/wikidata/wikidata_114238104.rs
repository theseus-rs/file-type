use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114238104: FileFormat = FileFormat {
    id: 114_238_104,
    puid: "wikidata/114238104",
    name: "Netscape packetized audio",
    extensions: &["la", "lma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
