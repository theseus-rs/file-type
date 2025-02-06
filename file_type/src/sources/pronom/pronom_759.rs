use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_759: FileFormat = FileFormat {
    id: 759,
    source_type: SourceType::Pronom,
    name: "StarOffice Impress",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
