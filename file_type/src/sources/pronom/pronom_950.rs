use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_950: FileFormat = FileFormat {
    id: 950,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet for Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
