use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1420: FileFormat = FileFormat {
    id: 2_238,
    puid: "fmt/1420",
    name: "Corel Print House/Print Office Document",
    extensions: &["cph", "cpd", "cpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
