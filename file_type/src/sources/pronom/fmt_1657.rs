use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1657: FileFormat = FileFormat {
    id: 2_484,
    puid: "fmt/1657",
    name: "XIMG (Extended GEM Bit Image)",
    extensions: &["ximg", "img"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x49, 0x4D, 0x47, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 223,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
