use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1489: FileFormat = FileFormat {
    id: 2_312,
    puid: "fmt/1489",
    name: "Phantom CINE Compressed Video File",
    extensions: &["cci"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x43, 0x49]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x01]),
                    Token::WildcardCount(1),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
