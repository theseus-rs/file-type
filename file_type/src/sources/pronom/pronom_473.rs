use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_473: FileFormat = FileFormat {
    id: 473,
    source_type: SourceType::Pronom,
    name: "Digital Terrain Elevation Data",
    extensions: &["dted", "dt0", "dt1", "dt2", "avg", "min", "max"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x55, 0x48, 0x4C, 0x31]),
                    Token::WildcardCount(7),
                    Token::Any(&[&[Token::Literal(&[0x57])], &[Token::Literal(&[0x45])]]),
                    Token::WildcardCount(7),
                    Token::Any(&[&[Token::Literal(&[0x4E])], &[Token::Literal(&[0x53])]]),
                    Token::WildcardCount(60),
                    Token::Literal(&[0x44, 0x53, 0x49]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
