use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2479: FileFormat = FileFormat {
    id: 2_479,
    source_type: SourceType::Pronom,
    name: "Typescript",
    extensions: &["ts", "tsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
