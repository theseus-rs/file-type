use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_857: FileFormat = FileFormat {
    id: 857,
    source_type: SourceType::Pronom,
    name: "Revit Family File",
    extensions: &["rfa"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 767,
    }],
};
