use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1974: FileFormat = FileFormat {
    id: 1_974,
    source_type: SourceType::Pronom,
    name: "Praat Picture File",
    extensions: &["prapic"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x72, 0x61, 0x61, 0x74, 0x50, 0x69, 0x63, 0x74, 0x75, 0x72, 0x65, 0x46,
                    0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
