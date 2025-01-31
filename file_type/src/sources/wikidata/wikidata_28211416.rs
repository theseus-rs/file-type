use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28211416: FileFormat = FileFormat {
    id: 28_211_416,
    puid: "wikidata/28211416",
    name: "Ability Write",
    extensions: &["awp", "aww"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
