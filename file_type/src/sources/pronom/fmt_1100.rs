use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1100: FileFormat = FileFormat {
    id: 1_908,
    puid: "fmt/1100",
    name: "yEnc Encoded File",
    extensions: &["yenc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3D, 0x79, 0x62, 0x65, 0x67, 0x69, 0x6E, 0x20, 0x6C, 0x69, 0x6E, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
