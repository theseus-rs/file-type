use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_113: FileFormat = FileFormat {
    id: 677,
    puid: "fmt/113",
    name: "Still Picture Interchange File Format",
    extensions: &[],
    media_types: &["image/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
