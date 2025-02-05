use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2171: FileFormat = FileFormat {
    id: 2_171,
    source_type: SourceType::Pronom,
    name: "FamilyTree Maker Database",
    extensions: &["ftw", "fbk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
