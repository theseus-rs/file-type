use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1492: FileFormat = FileFormat {
    id: 2_315,
    puid: "fmt/1492",
    name: "Harvard Graphics Presentation",
    extensions: &["pr4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
