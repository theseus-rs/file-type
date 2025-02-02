use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_863: FileFormat = FileFormat {
    id: 863,
    source_type: SourceType::Pronom,
    name: "Revit Workspace",
    extensions: &["rws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
