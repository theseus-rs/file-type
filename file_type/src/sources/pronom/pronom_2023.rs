use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2023: FileFormat = FileFormat {
    id: 2_023,
    source_type: SourceType::Pronom,
    name: "Zoner Callisto Metafile",
    extensions: &["zmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
