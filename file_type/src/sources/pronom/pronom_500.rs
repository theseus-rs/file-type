use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_500: FileFormat = FileFormat {
    id: 500,
    source_type: SourceType::Pronom,
    name: "Lotus Notes Database",
    extensions: &["ns3", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
