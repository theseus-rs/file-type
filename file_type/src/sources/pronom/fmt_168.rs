use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_168: FileFormat = FileFormat {
    id: 896,
    puid: "fmt/168",
    name: "Microsoft Works Spreadsheet for DOS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
