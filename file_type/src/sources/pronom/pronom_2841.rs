use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2841: FileFormat = FileFormat {
    id: 2_841,
    source_type: SourceType::Pronom,
    name: "Finale Notation File",
    extensions: &["musx"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
