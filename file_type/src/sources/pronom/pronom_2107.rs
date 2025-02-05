use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2107: FileFormat = FileFormat {
    id: 2_107,
    source_type: SourceType::Pronom,
    name: "RFFlow Chart",
    extensions: &["flo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x46, 0x4C, 0x4F, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
