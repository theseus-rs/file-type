use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1007: FileFormat = FileFormat {
    id: 1_007,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for Macintosh",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
