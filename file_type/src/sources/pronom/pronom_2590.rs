use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2590: FileFormat = FileFormat {
    id: 2_590,
    source_type: SourceType::Pronom,
    name: "Psion Series 3 Bitmap",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0xDC, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
