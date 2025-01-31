use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_103: FileFormat = FileFormat {
    id: 151,
    puid: "x-fmt/103",
    name: "AutoCAD Compiled Shape/Font File",
    extensions: &["shx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x2D, 0x38, 0x36, 0x20,
                    ]),
                    Token::Any(&[
                        &[Token::Literal(&[0x73, 0x68, 0x61, 0x70, 0x65, 0x73])],
                        &[Token::Literal(&[0x75, 0x6E, 0x69, 0x66, 0x6F, 0x6E, 0x74])],
                        &[Token::Literal(&[0x62, 0x69, 0x67, 0x66, 0x6F, 0x6E, 0x74])],
                    ]),
                    Token::Literal(&[0x20, 0x31, 0x2E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
