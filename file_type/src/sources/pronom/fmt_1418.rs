use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1418: FileFormat = FileFormat {
    id: 2_236,
    puid: "fmt/1418",
    name: "Corel Print House Document",
    extensions: &["cph", "cpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
