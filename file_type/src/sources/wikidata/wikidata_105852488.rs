use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852488: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_488,
        source_type: SourceType::Wikidata,
        name: "Shadowgrounds 3D model",
        extensions: &["s3d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x33, 0x44, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
