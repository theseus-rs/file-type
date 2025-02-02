use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2448: FileFormat = FileFormat {
    id: 2_448,
    source_type: SourceType::Pronom,
    name: "AHX-Module Format (formerly THX module format)",
    extensions: &["ahx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
