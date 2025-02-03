use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28211416: FileFormat = FileFormat {
    id: 28_211_416,
    source_type: SourceType::Wikidata,
    name: "Ability Write",
    extensions: &["awp", "aww"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
