use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_58: FileFormat = FileFormat {
    id: 95,
    puid: "x-fmt/58",
    name: "Microsoft Excel Web Query",
    extensions: &["iqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
