use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84842911: FileType = FileType {
    file_format: &FileFormat {
        id: 84_842_911,
        source_type: SourceType::Wikidata,
        name: "GL Transmission Format (Binary)",
        extensions: &["glb"],
        media_types: &["application/octet-stream", "model/gltf-binary"],
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
    },
};
