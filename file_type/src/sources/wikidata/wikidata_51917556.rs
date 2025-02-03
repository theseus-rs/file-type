use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51917556: FileFormat = FileFormat {
    id: 51_917_556,
    source_type: SourceType::Wikidata,
    name: "WordStar for Windows Document",
    extensions: &["ws", "wsd", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
