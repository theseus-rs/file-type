use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2245: FileFormat = FileFormat {
    id: 2_245,
    source_type: SourceType::Pronom,
    name: "MacDraw",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x52, 0x57, 0x47]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00, 0x00])],
                        &[Token::Literal(&[0x44, 0x32])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
