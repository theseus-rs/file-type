use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2726: FileFormat = FileFormat {
    id: 2_726,
    source_type: SourceType::Pronom,
    name: "Guitar Pro File",
    extensions: &["gtp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x19, 0x46, 0x49, 0x43, 0x48, 0x49, 0x45, 0x52, 0x20, 0x47, 0x55, 0x49, 0x54,
                    0x41, 0x52, 0x45, 0x20, 0x50, 0x52, 0x4F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
