use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1442: FileFormat = FileFormat {
    id: 1_442,
    source_type: SourceType::Pronom,
    name: "ASTM E57 3D File Format",
    extensions: &["e57"],
    media_types: &["model/e57"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x53, 0x54, 0x4D, 0x2D, 0x45, 0x35, 0x37,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
