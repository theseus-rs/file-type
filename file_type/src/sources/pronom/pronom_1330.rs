use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1330: FileFormat = FileFormat {
    id: 1_330,
    source_type: SourceType::Pronom,
    name: "GEM Metafile Format",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0xFF, 0x18, 0x00, 0x36, 0x01]),
                    Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x00])]]),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_329,
    }],
};
