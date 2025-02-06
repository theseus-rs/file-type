use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1883: FileFormat = FileFormat {
    id: 1_883,
    source_type: SourceType::Pronom,
    name: "AVCHD Index File",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x4E, 0x44, 0x58, 0x30, 0x31, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
