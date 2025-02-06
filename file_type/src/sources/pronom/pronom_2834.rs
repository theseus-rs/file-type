use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2834: FileFormat = FileFormat {
    id: 2_834,
    source_type: SourceType::Pronom,
    name: "Atrac Codec File",
    extensions: &["aea"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2_896),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC])],
                },
            },
        ],
    }],
    related_formats: &[],
};
