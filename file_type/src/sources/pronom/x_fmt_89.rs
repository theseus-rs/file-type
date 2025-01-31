use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_89: FileFormat = FileFormat {
    id: 136,
    puid: "x-fmt/89",
    name: "Freelance File",
    extensions: &["pre"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
