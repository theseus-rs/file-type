use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1744: FileFormat = FileFormat {
    id: 2_590,
    puid: "fmt/1744",
    name: "Psion Series 3 Bitmap",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0xDC, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
