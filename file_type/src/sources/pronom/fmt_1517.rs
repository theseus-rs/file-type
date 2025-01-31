use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1517: FileFormat = FileFormat {
    id: 2_341,
    puid: "fmt/1517",
    name: "Serif PhotoPlus Image",
    extensions: &["spp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
