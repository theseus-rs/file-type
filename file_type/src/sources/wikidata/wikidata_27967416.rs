use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967416: FileFormat = FileFormat {
    id: 27_967_416,
    source_type: SourceType::Wikidata,
    name: "Voice Sequence",
    extensions: &["vsq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
