use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1412: FileFormat = FileFormat {
    id: 2_230,
    puid: "fmt/1412",
    name: "Flow Charting Graphic Flowcharting Image",
    extensions: &["gfi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0C, 0xAF, 0xCB, 0xED])],
            },
        }],
    }],
    related_formats: &[],
};
