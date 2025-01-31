use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1518: FileFormat = FileFormat {
    id: 2_342,
    puid: "fmt/1518",
    name: "Serif PhotoPlus Image",
    extensions: &["spp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
