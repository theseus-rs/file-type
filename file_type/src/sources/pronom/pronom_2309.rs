use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2309: FileFormat = FileFormat {
    id: 2_309,
    source_type: SourceType::Pronom,
    name: "Novell Address Book",
    extensions: &["nab"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0x3A, 0x3A, 0x54, 0x41, 0x47, 0x4D, 0x41, 0x50, 0x3A, 0x3A, 0x3A, 0x30,
                    0x46, 0x46, 0x45, 0x30, 0x30, 0x30, 0x33, 0x3A, 0x2A, 0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
