use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967416: FileFormat = FileFormat {
    id: 27_967_416,
    puid: "wikidata/27967416",
    name: "Voice Sequence",
    extensions: &["vsq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
