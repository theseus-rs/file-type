use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_89: FileFormat = FileFormat {
    id: 89,
    source_type: SourceType::Pronom,
    name: "Frame Vector Metafile",
    extensions: &["fmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
