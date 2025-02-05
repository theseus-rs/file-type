use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1427: FileFormat = FileFormat {
    id: 1_427,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Macro-Enabled Add-In",
    extensions: &["xlam"],
    media_types: &["application/vnd.ms-excel.addin.macroEnabled.12"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 940,
    }],
};
