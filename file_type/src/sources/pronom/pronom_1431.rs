use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1431: FileFormat = FileFormat {
    id: 1_431,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint Macro-Enabled Template",
    extensions: &["potm"],
    media_types: &["application/vnd.ms-powerpoint.template.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 941,
    }],
};
