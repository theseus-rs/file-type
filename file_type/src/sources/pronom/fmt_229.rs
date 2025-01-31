use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_229: FileFormat = FileFormat {
    id: 959,
    puid: "fmt/229",
    name: "Microsoft Works Spreadsheet for Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
