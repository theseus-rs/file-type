use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_118: FileFormat = FileFormat {
    id: 170,
    puid: "x-fmt/118",
    name: "Microsoft Works Spreadsheet",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
