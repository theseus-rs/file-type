use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860703: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_703,
        source_type: SourceType::Wikidata,
        name: "Sparse Voxel Octree (binary) (v1)",
        extensions: &["rsvo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x56, 0x4F, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
