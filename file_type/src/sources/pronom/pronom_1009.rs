use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1009: FileFormat = FileFormat {
    id: 1_009,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet for Macintosh",
    extensions: &["wks"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
