use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1002: FileFormat = FileFormat {
    id: 1_002,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet for DOS",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
