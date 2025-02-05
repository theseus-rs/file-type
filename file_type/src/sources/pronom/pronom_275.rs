use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_275: FileFormat = FileFormat {
    id: 275,
    source_type: SourceType::Pronom,
    name: "CCITT G.711 Audio",
    extensions: &["ulaw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
