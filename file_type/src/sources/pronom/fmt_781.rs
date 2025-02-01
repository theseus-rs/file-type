use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_781: FileFormat = FileFormat {
    id: 1_580,
    puid: "fmt/781",
    name: "Snoop Packet Capture",
    extensions: &["snoop"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x6E, 0x6F, 0x6F, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
