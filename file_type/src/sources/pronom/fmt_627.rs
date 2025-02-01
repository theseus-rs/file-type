use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_627: FileFormat = FileFormat {
    id: 1_426,
    puid: "fmt/627",
    name: "Microsoft Excel Macro-Enabled Template",
    extensions: &["xltm"],
    media_types: &["application/vnd.ms-excel.template.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 940,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
