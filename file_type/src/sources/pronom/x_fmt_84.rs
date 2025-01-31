use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_84: FileFormat = FileFormat {
    id: 129,
    puid: "x-fmt/84",
    name: "Microsoft Powerpoint Design Template",
    extensions: &["pot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
