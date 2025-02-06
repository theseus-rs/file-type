use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2240: FileFormat = FileFormat {
    id: 2_240,
    source_type: SourceType::Pronom,
    name: "Corel Photo House Image",
    extensions: &["cps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
