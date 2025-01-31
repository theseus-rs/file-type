use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_345: FileFormat = FileFormat {
    id: 509,
    puid: "x-fmt/345",
    name: "Microsoft Works Document",
    extensions: &["bps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
