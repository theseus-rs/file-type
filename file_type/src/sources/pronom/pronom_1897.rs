use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1897: FileFormat = FileFormat {
    id: 1_897,
    source_type: SourceType::Pronom,
    name: "VBScript (VBS) File",
    extensions: &["vbs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
