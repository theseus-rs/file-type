use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1979: FileFormat = FileFormat {
    id: 1_979,
    source_type: SourceType::Pronom,
    name: "Maya IFF Image File",
    extensions: &["iff", "ico"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x34]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x43, 0x49, 0x4D, 0x47]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
