use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_246: FileFormat = FileFormat {
    id: 362,
    puid: "x-fmt/246",
    name: "Microsoft Project",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
