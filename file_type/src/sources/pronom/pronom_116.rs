use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_116: FileFormat = FileFormat {
    id: 116,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel OLAP Query",
    extensions: &["oqy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
