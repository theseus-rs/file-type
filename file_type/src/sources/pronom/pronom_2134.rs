use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2134: FileFormat = FileFormat {
    id: 2_134,
    source_type: SourceType::Pronom,
    name: "GL Transmission Format (Binary)",
    extensions: &["glb"],
    media_types: &["model/gltf-binary"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x67, 0x6C, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
