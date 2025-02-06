use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_77: FileFormat = FileFormat {
    id: 77,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel ODBC Query",
    extensions: &["dqy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
