use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_628: FileFormat = FileFormat {
    id: 1_427,
    puid: "fmt/628",
    name: "Microsoft Excel Macro-Enabled Add-In",
    extensions: &["xlam"],
    media_types: &["application/vnd.ms-excel.addin.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 940,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
