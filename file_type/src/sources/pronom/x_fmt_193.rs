use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_193: FileFormat = FileFormat {
    id: 266,
    puid: "x-fmt/193",
    name: "Unisys (Sperry) System Data File",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
