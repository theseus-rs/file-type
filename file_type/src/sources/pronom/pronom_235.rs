use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_235: FileFormat = FileFormat {
    id: 235,
    source_type: SourceType::Pronom,
    name: "NAP Metafile",
    extensions: &["nap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
