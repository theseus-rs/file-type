use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_74: FileFormat = FileFormat {
    id: 116,
    puid: "x-fmt/74",
    name: "Microsoft Excel OLAP Query",
    extensions: &["oqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
