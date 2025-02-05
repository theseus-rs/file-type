use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1522: FileFormat = FileFormat {
    id: 1_522,
    source_type: SourceType::Pronom,
    name: "Farandole Composer Module",
    extensions: &["far"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x41, 0x52, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
