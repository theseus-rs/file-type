use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862204: FileFormat = FileFormat {
    id: 105_862_204,
    source_type: SourceType::Wikidata,
    name: "Multi Edit configuration",
    extensions: &["me", "mew"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x4D, 0x55, 0x4C, 0x54, 0x49, 0x2D, 0x45, 0x44, 0x49, 0x54, 0x20,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x4D, 0x55, 0x4C, 0x54, 0x49, 0x2D, 0x45, 0x44, 0x49, 0x54, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
