use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_116: FileFormat = FileFormat {
    id: 116,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel OLAP Query",
    extensions: &["oqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
