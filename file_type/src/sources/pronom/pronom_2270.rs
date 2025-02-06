use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2270: FileFormat = FileFormat {
    id: 2_270,
    source_type: SourceType::Pronom,
    name: "Aldus FreeHand Drawing",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x48, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
