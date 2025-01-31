use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1362: FileFormat = FileFormat {
    id: 2_180,
    puid: "fmt/1362",
    name: "Microsoft MapPoint Document",
    extensions: &["ptm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
