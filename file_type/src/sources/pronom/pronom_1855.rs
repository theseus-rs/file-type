use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1855: FileFormat = FileFormat {
    id: 1_855,
    source_type: SourceType::Pronom,
    name: "QuickDraw 3D Metafile (Binary)",
    extensions: &["3dmf"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_013,
    }],
};
