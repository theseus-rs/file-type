use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34748140: FileFormat = FileFormat {
    id: 34_748_140,
    source_type: SourceType::Wikidata,
    name: "TAP",
    extensions: &["tap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
