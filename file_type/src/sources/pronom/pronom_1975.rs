use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1975: FileFormat = FileFormat {
    id: 1_975,
    source_type: SourceType::Pronom,
    name: "Praat Script File",
    extensions: &["praat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
