use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2669: FileFormat = FileFormat {
    id: 2_669,
    source_type: SourceType::Pronom,
    name: "Direct Stream Digital Interchange File Format",
    extensions: &["dff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x52, 0x4D, 0x38]),
                    Token::WildcardCount(8),
                    Token::Literal(&[0x44, 0x53, 0x44, 0x20, 0x46, 0x56, 0x45, 0x52]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
