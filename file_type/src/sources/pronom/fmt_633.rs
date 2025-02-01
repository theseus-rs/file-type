use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_633: FileFormat = FileFormat {
    id: 1_432,
    puid: "fmt/633",
    name: "Microsoft PowerPoint Macro-Enabled Add-In",
    extensions: &["ppam"],
    media_types: &["application/vnd.ms-powerpoint.addin.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
