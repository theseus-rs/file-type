use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_893: FileFormat = FileFormat {
    id: 1_697,
    puid: "fmt/893",
    name: "i2 Analysts Notebook",
    extensions: &["anb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
