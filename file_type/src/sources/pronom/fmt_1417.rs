use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1417: FileFormat = FileFormat {
    id: 2_235,
    puid: "fmt/1417",
    name: "Corel Print House Document",
    extensions: &["cph", "cpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
