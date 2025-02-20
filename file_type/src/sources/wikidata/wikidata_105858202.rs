use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858202: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_202,
        source_type: SourceType::Wikidata,
        name: "EverQuest Game data",
        extensions: &["eqg", "pak", "pfs", "s3d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x53, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
