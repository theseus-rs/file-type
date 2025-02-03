use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1391: FileFormat = FileFormat {
    id: 1_391,
    source_type: SourceType::Pronom,
    name: "Microsoft Word Macro-Enabled Document Template",
    extensions: &["dotm"],
    media_types: &["application/vnd.ms-word.template.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 1_160,
    }],
};
