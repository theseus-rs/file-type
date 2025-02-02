use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34748140: FileFormat = FileFormat {
    id: 34_748_140,
    source_type: SourceType::Wikidata,
    name: "TAP",
    extensions: &["tap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
