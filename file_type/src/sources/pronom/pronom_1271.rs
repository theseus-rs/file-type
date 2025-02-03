use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1271: FileFormat = FileFormat {
    id: 1_271,
    source_type: SourceType::Pronom,
    name: "7Zip format",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
