use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117835826: FileFormat = FileFormat {
    id: 117_835_826,
    source_type: SourceType::Wikidata,
    name: "Fujitsu DexNET file",
    extensions: &["dxn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
