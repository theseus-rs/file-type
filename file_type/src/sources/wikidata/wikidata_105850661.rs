use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850661: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_661,
        source_type: SourceType::Wikidata,
        name: "Voxlap voxel sprite",
        extensions: &["kv6"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x76, 0x78, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
