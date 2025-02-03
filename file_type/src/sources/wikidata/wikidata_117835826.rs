use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117835826: FileFormat = FileFormat {
    id: 117_835_826,
    source_type: SourceType::Wikidata,
    name: "Fujitsu DexNET file",
    extensions: &["dxn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
