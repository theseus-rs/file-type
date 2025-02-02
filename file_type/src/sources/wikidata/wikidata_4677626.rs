use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4677626: FileFormat = FileFormat {
    id: 4_677_626,
    source_type: SourceType::Wikidata,
    name: "Activity Streams",
    extensions: &["json"],
    media_types: &["application/activity+json"],
    internal_signatures: &[],
    related_formats: &[],
};
