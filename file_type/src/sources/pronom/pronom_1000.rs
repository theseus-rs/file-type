use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1000: FileFormat = FileFormat {
    id: 1_000,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet for DOS",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
