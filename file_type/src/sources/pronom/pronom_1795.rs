use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1795: FileFormat = FileFormat {
    id: 1_795,
    source_type: SourceType::Pronom,
    name: "ESRI File Geodatabase",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
