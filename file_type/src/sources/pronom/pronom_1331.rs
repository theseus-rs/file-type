use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1331: FileFormat = FileFormat {
    id: 1_331,
    source_type: SourceType::Pronom,
    name: "Macromedia FreeHand",
    extensions: &["fh7"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
