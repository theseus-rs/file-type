use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1397: FileFormat = FileFormat {
    id: 2_215,
    puid: "fmt/1397",
    name: "FARO Laser Scan File",
    extensions: &["fls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
