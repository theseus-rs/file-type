use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_526: FileFormat = FileFormat {
    id: 526,
    source_type: SourceType::Pronom,
    name: "StarOffice Impress",
    extensions: &["sdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
