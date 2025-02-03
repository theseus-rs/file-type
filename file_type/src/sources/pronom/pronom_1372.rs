use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1372: FileFormat = FileFormat {
    id: 1_372,
    source_type: SourceType::Pronom,
    name: "Windows Media Metafile",
    extensions: &["wmx", "wax", "wvx", "asx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3C]),
                    Token::Any(&[
                        &[Token::Literal(&[0x61, 0x73, 0x78])],
                        &[Token::Literal(&[0x41, 0x53, 0x58])],
                    ]),
                    Token::Literal(&[0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E])],
                        &[Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E])],
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x3D]),
                    Token::WildcardCountRange(0, 3),
                    Token::Literal(&[0x33]),
                    Token::WildcardCountRange(0, 64),
                    Token::Literal(&[0x3E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
