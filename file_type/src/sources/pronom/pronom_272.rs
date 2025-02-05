use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_272: FileFormat = FileFormat {
    id: 272,
    source_type: SourceType::Pronom,
    name: "Pagemaker TableEditor Graphics",
    extensions: &["tbl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
