use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1216: FileFormat = FileFormat {
    id: 2_026,
    puid: "fmt/1216",
    name: "Lotus Freelance Show",
    extensions: &["prz"],
    media_types: &["application/vnd.lotus-freelance"],
    internal_signatures: &[InternalSignature {
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
