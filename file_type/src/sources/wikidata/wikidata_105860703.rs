use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860703: FileFormat = FileFormat {
    id: 105_860_703,
    puid: "wikidata/105860703",
    name: "Sparse Voxel Octree (binary) (v1)",
    extensions: &["rsvo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x56, 0x4F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
