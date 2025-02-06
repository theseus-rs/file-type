use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51802416: FileFormat = FileFormat {
    id: 51_802_416,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator Plus Data File",
    extensions: &["cce"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
