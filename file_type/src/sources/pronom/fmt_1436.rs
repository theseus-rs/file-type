use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1436: FileFormat = FileFormat {
    id: 2_254,
    puid: "fmt/1436",
    name: "Minitab Project",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
