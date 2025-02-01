use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1237: FileFormat = FileFormat {
    id: 2_055,
    puid: "fmt/1237",
    name: "FileMaker Pro Database",
    extensions: &["fmp12"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xC0, 0x48, 0x42, 0x41, 0x4D, 0x37]),
                    Token::WildcardCountRange(0, 512),
                    Token::Literal(&[
                        0x48, 0x42, 0x41, 0x4D, 0x32, 0x31, 0x32, 0x35, 0x4A, 0x41, 0x4E, 0x31,
                        0x31, 0xC1, 0x02, 0x48, 0x08, 0x50, 0x72, 0x6F, 0x20, 0x31, 0x32, 0x2E,
                        0x30, 0xC0, 0xC0,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
