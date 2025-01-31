use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1316: FileFormat = FileFormat {
    id: 2_134,
    puid: "fmt/1316",
    name: "GL Transmission Format (Binary)",
    extensions: &["glb"],
    media_types: &["model/gltf-binary"],
    internal_signatures: &[InternalSignature {
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
