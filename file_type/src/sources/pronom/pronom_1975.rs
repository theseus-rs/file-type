use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1975: FileFormat = FileFormat {
    id: 1_975,
    source_type: SourceType::Pronom,
    name: "Praat Script File",
    extensions: &["praat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
