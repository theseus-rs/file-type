use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967416: FileFormat = FileFormat {
    id: 27_967_416,
    source_type: SourceType::Wikidata,
    name: "Voice Sequence",
    extensions: &["vsq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
