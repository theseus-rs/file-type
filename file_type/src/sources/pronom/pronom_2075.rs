use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2075: FileFormat = FileFormat {
    id: 2_075,
    source_type: SourceType::Pronom,
    name: "AutoCAD Temporary File",
    extensions: &["ac$"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
