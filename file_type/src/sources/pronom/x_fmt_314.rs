use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_314: FileFormat = FileFormat {
    id: 473,
    puid: "x-fmt/314",
    name: "Digital Terrain Elevation Data",
    extensions: &["dted", "dt0", "dt1", "dt2", "avg", "min", "max"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
