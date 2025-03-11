use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852029: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_029,
        source_type: SourceType::Wikidata,
        name: "Sereal serialized data (v1-2)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0x73, 0x72, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
