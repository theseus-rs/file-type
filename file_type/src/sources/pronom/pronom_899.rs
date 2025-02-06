use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_899: FileFormat = FileFormat {
    id: 899,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for DOS",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
