use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1303: FileFormat = FileFormat {
    id: 2_121,
    puid: "fmt/1303",
    name: "Microsoft Shell Scrap Object File",
    extensions: &["shs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
