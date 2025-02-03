use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84842911: FileFormat = FileFormat {
    id: 84_842_911,
    source_type: SourceType::Wikidata,
    name: "GL Transmission Format (Binary)",
    extensions: &["glb"],
    media_types: &["application/octet-stream", "model/gltf-binary"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x67, 0x6C, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x67, 0x6C, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
