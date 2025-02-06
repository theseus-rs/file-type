use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46496687: FileFormat = FileFormat {
    id: 46_496_687,
    source_type: SourceType::Wikidata,
    name: "Resource Adapter Archive",
    extensions: &["rar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
