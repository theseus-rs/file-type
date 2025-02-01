use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_163: FileFormat = FileFormat {
    id: 235,
    puid: "x-fmt/163",
    name: "NAP Metafile",
    extensions: &["nap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
