use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_259: FileFormat = FileFormat {
    id: 377,
    puid: "x-fmt/259",
    name: "Microsoft Visio Drawing",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
