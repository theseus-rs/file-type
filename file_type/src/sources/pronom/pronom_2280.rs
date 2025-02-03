use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2280: FileFormat = FileFormat {
    id: 2_280,
    source_type: SourceType::Pronom,
    name: "OrgPlus File",
    extensions: &["opx", "opxt", "ops"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4F, 0x43, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
