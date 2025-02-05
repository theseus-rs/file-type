use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_499: FileFormat = FileFormat {
    id: 499,
    source_type: SourceType::Pronom,
    name: "Lotus Notes Database",
    extensions: &["ns2", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
