use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2033: FileFormat = FileFormat {
    id: 2_033,
    source_type: SourceType::Pronom,
    name: "PaperPort MAX",
    extensions: &["max"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x69, 0x47, 0x42, 0x65, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
