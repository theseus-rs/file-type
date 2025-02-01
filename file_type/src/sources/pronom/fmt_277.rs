use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_277: FileFormat = FileFormat {
    id: 1_017,
    puid: "fmt/277",
    name: "ESRI Arc/View Shapefile Index",
    extensions: &["shx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x00, 0x27, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0xE8, 0x03, 0x00, 0x00]),
                    Token::WildcardCount(68),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x32]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
