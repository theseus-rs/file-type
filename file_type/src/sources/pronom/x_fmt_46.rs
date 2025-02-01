use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_46: FileFormat = FileFormat {
    id: 77,
    puid: "x-fmt/46",
    name: "Microsoft Excel ODBC Query",
    extensions: &["dqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
