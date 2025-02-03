use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_170: FileFormat = FileFormat {
    id: 170,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
