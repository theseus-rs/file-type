use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1580: FileFormat = FileFormat {
    id: 1_580,
    source_type: SourceType::Pronom,
    name: "Snoop Packet Capture",
    extensions: &["snoop"],
    media_types: &[],
    signatures: &[Signature {
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
