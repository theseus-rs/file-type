use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1253: FileFormat = FileFormat {
    id: 1_253,
    source_type: SourceType::Pronom,
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x57, 0x4C, 0x6D, 0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0xED, 0x08])],
                        &[Token::Literal(&[0x25, 0x05])],
                    ]),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
