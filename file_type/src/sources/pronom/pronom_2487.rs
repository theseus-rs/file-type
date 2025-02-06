use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2487: FileFormat = FileFormat {
    id: 2_487,
    source_type: SourceType::Pronom,
    name: "Arts & Letters Clip Art Library",
    extensions: &["yal"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x55, 0x53, 0x54, 0x4F, 0x4D, 0x00])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF])],
                },
            },
        ],
    }],
    related_formats: &[],
};
