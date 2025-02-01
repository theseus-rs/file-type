use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1294: FileFormat = FileFormat {
    id: 2_112,
    puid: "fmt/1294",
    name: "602Tab Spreadsheet",
    extensions: &["wls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
