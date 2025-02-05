use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_863: FileFormat = FileFormat {
    id: 863,
    source_type: SourceType::Pronom,
    name: "Revit Workspace",
    extensions: &["rws"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
