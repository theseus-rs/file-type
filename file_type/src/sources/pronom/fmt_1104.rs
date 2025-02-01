use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1104: FileFormat = FileFormat {
    id: 1_912,
    puid: "fmt/1104",
    name: "Seattle FilmWorks SFW Image Format",
    extensions: &["sfw"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x39, 0x34, 0x41])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x39, 0x38, 0x41])],
                },
            }],
        },
    ],
    related_formats: &[],
};
