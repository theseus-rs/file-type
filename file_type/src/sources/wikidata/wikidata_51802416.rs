use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51802416: FileFormat = FileFormat {
    id: 51_802_416,
    puid: "wikidata/51802416",
    name: "Calendar Creator Plus Data File",
    extensions: &["cce"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
