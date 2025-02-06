use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2779: FileFormat = FileFormat {
    id: 2_779,
    source_type: SourceType::Pronom,
    name: "LiveCode Stack",
    extensions: &["rev", "livecode"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x56, 0x4F, 0x37, 0x30, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
