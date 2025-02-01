use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_526: FileFormat = FileFormat {
    id: 1_313,
    puid: "fmt/526",
    name: "Adobe Font List",
    extensions: &["lst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x21, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x46, 0x6F, 0x6E, 0x74, 0x4C,
                    0x69, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
