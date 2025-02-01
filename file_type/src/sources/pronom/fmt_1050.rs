use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1050: FileFormat = FileFormat {
    id: 1_855,
    puid: "fmt/1050",
    name: "QuickDraw 3D Metafile (Binary)",
    extensions: &["3dmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x4D, 0x46, 0x00, 0x00, 0x00, 0x10, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_013,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
