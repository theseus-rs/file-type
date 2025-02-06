use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1622: FileFormat = FileFormat {
    id: 1_622,
    source_type: SourceType::Pronom,
    name: "CRT C64 Cartridge Image Format",
    extensions: &["crt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x43, 0x36, 0x34, 0x20, 0x43, 0x41, 0x52, 0x54, 0x52, 0x49, 0x44, 0x47,
                        0x45, 0x20, 0x20, 0x20,
                    ]),
                    Token::WildcardCount(48),
                    Token::Literal(&[0x43, 0x48, 0x49, 0x50]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
