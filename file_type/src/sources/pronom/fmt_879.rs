use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_879: FileFormat = FileFormat {
    id: 1_683,
    puid: "fmt/879",
    name: "Fortran",
    extensions: &["f90", "f95", "f03", "f", "for"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
