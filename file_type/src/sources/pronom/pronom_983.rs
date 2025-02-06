use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_983: FileFormat = FileFormat {
    id: 983,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet for Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
