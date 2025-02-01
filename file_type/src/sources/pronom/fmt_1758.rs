use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1758: FileFormat = FileFormat {
    id: 2_607,
    puid: "fmt/1758",
    name: "Media Descriptor File",
    extensions: &["mdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
