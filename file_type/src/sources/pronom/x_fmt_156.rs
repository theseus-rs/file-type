use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_156: FileFormat = FileFormat {
    id: 219,
    puid: "x-fmt/156",
    name: "Ventura Publisher",
    extensions: &["gen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
