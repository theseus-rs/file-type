use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1432: FileFormat = FileFormat {
    id: 1_432,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint Macro-Enabled Add-In",
    extensions: &["ppam"],
    media_types: &["application/vnd.ms-powerpoint.addin.macroEnabled.12"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 941,
    }],
};
