use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857114: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_114,
        source_type: SourceType::Wikidata,
        name: "GameCubeZip image",
        extensions: &["gcz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xC0, 0x0B, 0xB1])],
                },
            }],
        }],
        related_formats: &[],
    },
};
