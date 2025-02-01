use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_143: FileFormat = FileFormat {
    id: 204,
    puid: "x-fmt/143",
    name: "OS/2 Change Control File",
    extensions: &["cin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
