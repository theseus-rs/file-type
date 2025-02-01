use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_197: FileFormat = FileFormat {
    id: 271,
    puid: "x-fmt/197",
    name: "DataFlex Query Tag Name",
    extensions: &["tag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
