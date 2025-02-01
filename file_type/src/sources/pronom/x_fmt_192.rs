use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_192: FileFormat = FileFormat {
    id: 265,
    puid: "x-fmt/192",
    name: "SAS for MS-DOS Catalog",
    extensions: &["sct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
