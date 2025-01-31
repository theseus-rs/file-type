use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1088: FileFormat = FileFormat {
    id: 1_896,
    puid: "fmt/1088",
    name: "Visual Basic (VB) File",
    extensions: &["vb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
