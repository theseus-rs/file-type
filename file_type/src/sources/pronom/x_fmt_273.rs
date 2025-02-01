use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_273: FileFormat = FileFormat {
    id: 405,
    puid: "x-fmt/273",
    name: "Microsoft Word for MS-DOS Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
