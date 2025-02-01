use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_253: FileFormat = FileFormat {
    id: 991,
    puid: "fmt/253",
    name: "Microsoft Works Spreadsheet for Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
