use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2544: FileFormat = FileFormat {
    id: 2_544,
    source_type: SourceType::Pronom,
    name: "Persuasion Player File",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x39, 0x61])],
                },
            },
            ByteSequence {
                position_type: PositionType::Variable,
                offset: None,
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x6C, 0x70, 0x32, 0x52, 0x54, 0x50, 0x46,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
