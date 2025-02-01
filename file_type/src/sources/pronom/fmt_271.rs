use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_271: FileFormat = FileFormat {
    id: 1_009,
    puid: "fmt/271",
    name: "Microsoft Works Spreadsheet for Macintosh",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
