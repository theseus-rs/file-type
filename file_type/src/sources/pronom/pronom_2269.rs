use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2269: FileFormat = FileFormat {
    id: 2_269,
    source_type: SourceType::Pronom,
    name: "Aldus FreeHand Drawing",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x63, 0x66, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
