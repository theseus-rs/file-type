use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_272: FileFormat = FileFormat {
    id: 272,
    source_type: SourceType::Pronom,
    name: "Pagemaker TableEditor Graphics",
    extensions: &["tbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
