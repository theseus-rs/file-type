use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_129: FileFormat = FileFormat {
    id: 188,
    puid: "x-fmt/129",
    name: "Microsoft Word for Macintosh Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
