use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_125: FileFormat = FileFormat {
    id: 125,
    source_type: SourceType::Pronom,
    name: "Inkwriter/Notetaker Template",
    extensions: &["pdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
