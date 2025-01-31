use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_332: FileFormat = FileFormat {
    id: 495,
    puid: "x-fmt/332",
    name: "Lotus 1-2-3 Spreadsheet Formatting File",
    extensions: &["fm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
