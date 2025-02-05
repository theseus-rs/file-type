use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2230: FileFormat = FileFormat {
    id: 2_230,
    source_type: SourceType::Pronom,
    name: "Flow Charting Graphic Flowcharting Image",
    extensions: &["gfi"],
    media_types: &[],
    signatures: &[Signature {
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
