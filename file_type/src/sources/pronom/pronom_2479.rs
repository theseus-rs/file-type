use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2479: FileFormat = FileFormat {
    id: 2_479,
    source_type: SourceType::Pronom,
    name: "Typescript",
    extensions: &["ts", "tsx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
