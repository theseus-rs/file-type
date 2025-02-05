use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_170: FileFormat = FileFormat {
    id: 170,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
