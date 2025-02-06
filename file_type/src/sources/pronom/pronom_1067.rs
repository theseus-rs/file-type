use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1067: FileFormat = FileFormat {
    id: 1_067,
    source_type: SourceType::Pronom,
    name: "Portable Form File",
    extensions: &["pff"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x46, 0x46, 0x00, 0x07, 0x02, 0x00, 0x06,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
