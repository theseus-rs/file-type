use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_150: FileFormat = FileFormat {
    id: 793,
    puid: "fmt/150",
    name: "JPEG-LS",
    extensions: &["jls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
