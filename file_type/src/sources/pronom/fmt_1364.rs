use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1364: FileFormat = FileFormat {
    id: 2_182,
    puid: "fmt/1364",
    name: "V-Ray Material",
    extensions: &["vismat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3C, 0x76, 0x69, 0x73, 0x6D, 0x61, 0x74, 0x3E]),
                    Token::WildcardCountRange(0, 128),
                    Token::Literal(&[
                        0x72, 0x65, 0x6E, 0x64, 0x65, 0x72, 0x65, 0x72, 0x3D, 0x22, 0x76, 0x72,
                        0x61, 0x79, 0x22, 0x3E,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
