use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_904: FileFormat = FileFormat {
    id: 1_709,
    puid: "fmt/904",
    name: "Bluetooth Snoop Packet Capture",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x62, 0x74, 0x73, 0x6E, 0x6F, 0x6F, 0x70, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
