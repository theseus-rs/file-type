use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_201: FileFormat = FileFormat {
    id: 275,
    puid: "x-fmt/201",
    name: "CCITT G.711 Audio",
    extensions: &["ulaw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
