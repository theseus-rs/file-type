use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1978: FileFormat = FileFormat {
    id: 1_978,
    source_type: SourceType::Pronom,
    name: "Maya Icons or Swatches file",
    extensions: &["icons", "swatches"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x61, 0x79, 0x61, 0x49, 0x63, 0x6F, 0x6E, 0x73]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x53, 0x77, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
