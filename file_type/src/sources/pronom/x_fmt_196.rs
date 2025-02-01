use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_196: FileFormat = FileFormat {
    id: 269,
    puid: "x-fmt/196",
    name: "NeXt Sound",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
