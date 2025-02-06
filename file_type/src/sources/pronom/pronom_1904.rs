use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1904: FileFormat = FileFormat {
    id: 1_904,
    source_type: SourceType::Pronom,
    name: "FreeArc Archive Format",
    extensions: &["arc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x72, 0x43, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
