use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858632: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_632,
        source_type: SourceType::Wikidata,
        name: "Microsoft SQL Server Backup (compressed)",
        extensions: &["bak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x53, 0x51, 0x4C, 0x42, 0x41, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
