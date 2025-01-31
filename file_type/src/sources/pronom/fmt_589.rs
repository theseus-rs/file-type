use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_589: FileFormat = FileFormat {
    id: 1_381,
    puid: "fmt/589",
    name: "Windows Media Playlist",
    extensions: &["wpl"],
    media_types: &["application/vnd.ms-wpl"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x77, 0x70, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
