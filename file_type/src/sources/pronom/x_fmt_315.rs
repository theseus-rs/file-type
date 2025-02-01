use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_315: FileFormat = FileFormat {
    id: 474,
    puid: "x-fmt/315",
    name: "Document Type Definition",
    extensions: &["dtd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
