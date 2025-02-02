use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114238104: FileFormat = FileFormat {
    id: 114_238_104,
    source_type: SourceType::Wikidata,
    name: "Netscape packetized audio",
    extensions: &["la", "lma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
