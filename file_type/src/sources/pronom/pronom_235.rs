use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_235: FileFormat = FileFormat {
    id: 235,
    source_type: SourceType::Pronom,
    name: "NAP Metafile",
    extensions: &["nap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
