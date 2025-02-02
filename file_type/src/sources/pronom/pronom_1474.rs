use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1474: FileFormat = FileFormat {
    id: 1_474,
    source_type: SourceType::Pronom,
    name: "Serif PagePlus Publication",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
