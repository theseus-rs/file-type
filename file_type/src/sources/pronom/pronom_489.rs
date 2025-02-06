use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_489: FileFormat = FileFormat {
    id: 489,
    source_type: SourceType::Pronom,
    name: "Hewlett Packard AdvanceWrite Text File",
    extensions: &["aw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
