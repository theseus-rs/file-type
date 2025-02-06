use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_89: FileFormat = FileFormat {
    id: 89,
    source_type: SourceType::Pronom,
    name: "Frame Vector Metafile",
    extensions: &["fmv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
