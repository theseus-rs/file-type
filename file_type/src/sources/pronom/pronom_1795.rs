use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1795: FileFormat = FileFormat {
    id: 1_795,
    source_type: SourceType::Pronom,
    name: "ESRI File Geodatabase",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
