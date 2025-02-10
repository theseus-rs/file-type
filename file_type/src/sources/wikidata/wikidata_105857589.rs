use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857589: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_589,
        source_type: SourceType::Wikidata,
        name: "CP Backup Info (v7.x)",
        extensions: &["inf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x49, 0x43, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
