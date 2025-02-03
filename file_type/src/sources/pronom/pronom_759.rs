use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_759: FileFormat = FileFormat {
    id: 759,
    source_type: SourceType::Pronom,
    name: "StarOffice Impress",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
