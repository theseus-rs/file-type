use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_210: FileFormat = FileFormat {
    id: 936,
    puid: "fmt/210",
    name: "Statistica Report File",
    extensions: &["str"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
