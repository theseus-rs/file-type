use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_77: FileFormat = FileFormat {
    id: 77,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel ODBC Query",
    extensions: &["dqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
