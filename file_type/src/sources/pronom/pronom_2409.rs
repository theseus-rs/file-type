use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2409: FileFormat = FileFormat {
    id: 2_409,
    source_type: SourceType::Pronom,
    name: "ADRIFT Text Adventure File",
    extensions: &["taf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x42, 0x3F, 0xC9, 0x6A, 0x87, 0xC2, 0xCF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
