use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1374: FileFormat = FileFormat {
    id: 1_374,
    source_type: SourceType::Pronom,
    name: "LifeTechnologies SDS",
    extensions: &["sds"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x53, 0x32, 0x00, 0x02, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_381,
    }],
};
