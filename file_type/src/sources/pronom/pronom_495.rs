use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_495: FileFormat = FileFormat {
    id: 495,
    source_type: SourceType::Pronom,
    name: "Lotus 1-2-3 Spreadsheet Formatting File",
    extensions: &["fm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
