use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2310: FileFormat = FileFormat {
    id: 2_310,
    source_type: SourceType::Pronom,
    name: "Timeline Maker Document",
    extensions: &["tlm", "tlm3", "tlm4", "tlmp"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x37, 0x44, 0x30, 0x42, 0x44, 0x39, 0x32, 0x34, 0x41, 0x35, 0x38,
                        0x34, 0x32, 0x34, 0x38, 0x41, 0x37, 0x37, 0x35, 0x41, 0x43, 0x41, 0x37,
                        0x31, 0x32, 0x46, 0x43, 0x34, 0x30, 0x38, 0x34,
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
                        0x30, 0x36, 0x33, 0x35, 0x30, 0x35, 0x30, 0x31, 0x38, 0x35, 0x30, 0x46,
                        0x34, 0x39, 0x32, 0x36, 0x39, 0x38, 0x37, 0x32, 0x39, 0x32, 0x43, 0x38,
                        0x37, 0x43, 0x35, 0x33, 0x41, 0x44, 0x37, 0x46,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
