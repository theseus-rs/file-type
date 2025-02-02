use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_621: FileFormat = FileFormat {
    id: 621,
    source_type: SourceType::Pronom,
    name: "PCX",
    extensions: &["pcx", "pcc"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 622,
    }],
};
