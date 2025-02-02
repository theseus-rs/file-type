use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1996: FileFormat = FileFormat {
    id: 1_996,
    source_type: SourceType::Pronom,
    name: "Dr. Halo Image Palette",
    extensions: &["pal"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x48]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x0A, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
