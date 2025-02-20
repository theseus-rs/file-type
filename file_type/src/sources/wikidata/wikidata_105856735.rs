use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_735,
        source_type: SourceType::Wikidata,
        name: "UMT EFS Backup",
        extensions: &["ueb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x4D, 0x54, 0x45, 0x46, 0x53, 0x42, 0x41, 0x43, 0x4B, 0x55, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
