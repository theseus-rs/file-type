use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_305: FileFormat = FileFormat {
    id: 463,
    puid: "x-fmt/305",
    name: "Apple Sound",
    extensions: &["afc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
