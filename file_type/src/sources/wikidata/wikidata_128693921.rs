use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693921: FileFormat = FileFormat {
    id: 128_693_921,
    source_type: SourceType::Wikidata,
    name: "BQN file",
    extensions: &["bqn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
