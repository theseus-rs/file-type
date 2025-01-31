use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_936: FileFormat = FileFormat {
    id: 1_741,
    puid: "fmt/936",
    name: "Microsoft Picture It! Image File",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    internal_signatures: &[],
    related_formats: &[],
};
