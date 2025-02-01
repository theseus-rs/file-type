use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_721: FileFormat = FileFormat {
    id: 1_520,
    puid: "fmt/721",
    name: "VLW Font File",
    extensions: &["vlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
