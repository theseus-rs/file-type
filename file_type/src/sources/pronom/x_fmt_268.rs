use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_268: FileFormat = FileFormat {
    id: 388,
    puid: "x-fmt/268",
    name: "BZIP2 Compressed Archive",
    extensions: &["bz2"],
    media_types: &["application/x-bzip2"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x5A, 0x68]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x31, 0x41, 0x59, 0x26, 0x53, 0x59]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_878,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
