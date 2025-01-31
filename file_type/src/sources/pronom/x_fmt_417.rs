use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_417: FileFormat = FileFormat {
    id: 804,
    puid: "x-fmt/417",
    name: "HTML Extension File",
    extensions: &["htx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
