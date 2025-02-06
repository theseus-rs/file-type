use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1643: FileFormat = FileFormat {
    id: 1_643,
    source_type: SourceType::Pronom,
    name: "AccessData Custom Content Image",
    extensions: &["ad1", "ad2", "ad3", "ad4", "ad5"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x44, 0x53, 0x45, 0x47, 0x4D, 0x45, 0x4E, 0x54, 0x45, 0x44, 0x46, 0x49,
                    0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
