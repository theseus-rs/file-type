use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_899: FileFormat = FileFormat {
    id: 899,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for DOS",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
