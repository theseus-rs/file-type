use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1716: FileFormat = FileFormat {
    id: 2_552,
    puid: "fmt/1716",
    name: "Cintel Raw Image/DaVinci Resolve Image",
    extensions: &["cri", "dvcc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x44, 0x56, 0x43, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
