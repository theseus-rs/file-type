use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2484: FileFormat = FileFormat {
    id: 2_484,
    source_type: SourceType::Pronom,
    name: "XIMG (Extended GEM Bit Image)",
    extensions: &["ximg", "img"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x49, 0x4D, 0x47, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 223,
    }],
};
