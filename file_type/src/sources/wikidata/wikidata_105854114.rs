use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854114: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_114,
        source_type: SourceType::Wikidata,
        name: "1Password Cloud Keychain",
        extensions: &["attachment"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x50, 0x43, 0x4C, 0x44, 0x41, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
