use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_112: FileFormat = FileFormat {
    id: 112,
    source_type: SourceType::Pronom,
    name: "AutoCAD Source Menu File",
    extensions: &["mns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
