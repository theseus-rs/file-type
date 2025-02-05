use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2622: FileFormat = FileFormat {
    id: 2_622,
    source_type: SourceType::Pronom,
    name: "Casio QV CAM",
    extensions: &["cam"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x20, 0x4D, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
