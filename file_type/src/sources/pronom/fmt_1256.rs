use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1256: FileFormat = FileFormat {
    id: 2_074,
    puid: "fmt/1256",
    name: "MapInfo Workspace File",
    extensions: &["wor"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x21, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65]),
                    Token::WildcardCountRange(0, 6),
                    Token::Literal(&[0x21, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                    Token::WildcardCountRange(0, 6),
                    Token::Literal(&[0x21, 0x43, 0x68, 0x61, 0x72, 0x73, 0x65, 0x74]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
