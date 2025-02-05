use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1516: FileFormat = FileFormat {
    id: 1_516,
    source_type: SourceType::Pronom,
    name: "Scream Tracker Module",
    extensions: &["stm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(20),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x53, 0x63, 0x72, 0x65, 0x61, 0x6D, 0x21, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
