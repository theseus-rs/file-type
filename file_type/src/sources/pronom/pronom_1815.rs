use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1815: FileFormat = FileFormat {
    id: 1_815,
    source_type: SourceType::Pronom,
    name: "FBX (Filmbox) Text",
    extensions: &["fbx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x20, 0x46, 0x42, 0x58, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
