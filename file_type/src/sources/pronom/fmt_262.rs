use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_262: FileFormat = FileFormat {
    id: 1_000,
    puid: "fmt/262",
    name: "Microsoft Works Spreadsheet for DOS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
