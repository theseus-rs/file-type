use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2841: FileFormat = FileFormat {
    id: 2_841,
    source_type: SourceType::Pronom,
    name: "Finale Notation File",
    extensions: &["musx"],
    media_types: &["application/vnd.makemusic.notation"],
    signatures: &[],
    related_formats: &[],
};
