use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_501: FileFormat = FileFormat {
    id: 501,
    source_type: SourceType::Pronom,
    name: "Lotus Notes Database",
    extensions: &["ns4", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
