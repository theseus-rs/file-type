use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114238104: FileFormat = FileFormat {
    id: 114_238_104,
    source_type: SourceType::Wikidata,
    name: "Netscape packetized audio",
    extensions: &["la", "lma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
