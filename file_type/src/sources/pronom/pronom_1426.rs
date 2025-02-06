use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1426: FileFormat = FileFormat {
    id: 1_426,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Macro-Enabled Template",
    extensions: &["xltm"],
    media_types: &["application/vnd.ms-excel.template.macroEnabled.12"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 940,
    }],
};
