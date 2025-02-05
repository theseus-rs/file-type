use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2451: FileFormat = FileFormat {
    id: 2_451,
    source_type: SourceType::Pronom,
    name: "Art Of Noise",
    extensions: &["aon"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
