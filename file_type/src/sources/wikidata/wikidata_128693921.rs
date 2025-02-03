use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128693921: FileFormat = FileFormat {
    id: 128_693_921,
    source_type: SourceType::Wikidata,
    name: "BQN file",
    extensions: &["bqn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
