use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_209: FileFormat = FileFormat {
    id: 290,
    puid: "x-fmt/209",
    name: "SDSC Image Tool X Window Dump Format",
    extensions: &["xwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
