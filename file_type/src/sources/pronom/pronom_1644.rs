use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1644: FileFormat = FileFormat {
    id: 1_644,
    source_type: SourceType::Pronom,
    name: "AccessData Custom Content Image (Encrypted)",
    extensions: &["ad1", "ad2", "ad3", "ad4", "ad5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x43, 0x52, 0x59, 0x50, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
