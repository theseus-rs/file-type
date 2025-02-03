use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_500: FileFormat = FileFormat {
    id: 500,
    source_type: SourceType::Pronom,
    name: "Lotus Notes Database",
    extensions: &["ns3", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};
