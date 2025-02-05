use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1390: FileFormat = FileFormat {
    id: 1_390,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Template",
    extensions: &["xltx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 940,
    }],
};
