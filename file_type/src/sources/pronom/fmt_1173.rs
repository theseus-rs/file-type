use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1173: FileFormat = FileFormat {
    id: 1_983,
    puid: "fmt/1173",
    name: "FrameMD5",
    extensions: &["framemd5", "md5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x23, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3A, 0x20, 0x66, 0x72, 0x61,
                        0x6D, 0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6B, 0x73, 0x75, 0x6D, 0x73,
                    ]),
                    Token::WildcardCountRange(13, 16),
                    Token::Literal(&[0x23, 0x68, 0x61, 0x73, 0x68, 0x3A, 0x20, 0x4D, 0x44, 0x35]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
