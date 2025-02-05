use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2781: FileFormat = FileFormat {
    id: 2_781,
    source_type: SourceType::Pronom,
    name: "LiveCode Stack",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x56, 0x4F, 0x38, 0x30, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
