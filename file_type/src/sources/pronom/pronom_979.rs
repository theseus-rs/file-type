use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_979: FileFormat = FileFormat {
    id: 979,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet for Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
