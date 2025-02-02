use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2171: FileFormat = FileFormat {
    id: 2_171,
    source_type: SourceType::Pronom,
    name: "FamilyTree Maker Database",
    extensions: &["ftw", "fbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
