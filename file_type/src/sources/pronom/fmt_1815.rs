use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1815: FileFormat = FileFormat {
    id: 2_666,
    puid: "fmt/1815",
    name: "Adobe Color Swatch",
    extensions: &["aco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
