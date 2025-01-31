use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_247: FileFormat = FileFormat {
    id: 979,
    puid: "fmt/247",
    name: "Microsoft Works Spreadsheet for Windows",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
