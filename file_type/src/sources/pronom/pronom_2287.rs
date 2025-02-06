use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2287: FileFormat = FileFormat {
    id: 2_287,
    source_type: SourceType::Pronom,
    name: "Maestro Music File",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x65, 0x73, 0x74, 0x72, 0x6F, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
