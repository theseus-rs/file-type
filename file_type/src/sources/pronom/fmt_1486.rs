use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1486: FileFormat = FileFormat {
    id: 2_309,
    puid: "fmt/1486",
    name: "Novell Address Book",
    extensions: &["nab"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
