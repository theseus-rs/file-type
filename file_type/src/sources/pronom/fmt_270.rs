use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_270: FileFormat = FileFormat {
    id: 1_008,
    puid: "fmt/270",
    name: "Microsoft Works Spreadsheet for Macintosh",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
