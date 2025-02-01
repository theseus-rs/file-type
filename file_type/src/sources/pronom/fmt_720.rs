use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_720: FileFormat = FileFormat {
    id: 1_519,
    puid: "fmt/720",
    name: "MBOX",
    extensions: &["mbox"],
    media_types: &["application/mbox"],
    internal_signatures: &[],
    related_formats: &[],
};
