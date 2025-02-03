use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1807: FileFormat = FileFormat {
    id: 1_807,
    source_type: SourceType::Pronom,
    name: "Nearly Raw Raster Data",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4E, 0x52, 0x52, 0x44, 0x30, 0x30]),
                    Token::Any(&[
                        &[Token::Literal(&[0x2E, 0x30, 0x31])],
                        &[Token::Literal(&[0x30, 0x31])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_808,
    }],
};
