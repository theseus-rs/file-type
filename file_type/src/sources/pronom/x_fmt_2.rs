use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_2: FileFormat = FileFormat {
    id: 11,
    puid: "x-fmt/2",
    name: "Microsoft Word for Macintosh Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_091,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
