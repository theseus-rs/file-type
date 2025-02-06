use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1683: FileFormat = FileFormat {
    id: 1_683,
    source_type: SourceType::Pronom,
    name: "Fortran",
    extensions: &["f90", "f95", "f03", "f", "for"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
