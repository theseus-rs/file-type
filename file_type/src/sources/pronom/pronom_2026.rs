use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2026: FileFormat = FileFormat {
    id: 2_026,
    source_type: SourceType::Pronom,
    name: "Lotus Freelance Show",
    extensions: &["prz"],
    media_types: &["application/vnd.lotus-freelance"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x01, 0x0F, 0x00, 0x46, 0x52, 0x45, 0x45, 0x4C, 0x41, 0x4E, 0x43, 0x45,
                    0x04, 0x00, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
