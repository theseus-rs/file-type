use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_167: FileFormat = FileFormat {
    id: 895,
    puid: "fmt/167",
    name: "Microsoft Works Spreadsheet for DOS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
