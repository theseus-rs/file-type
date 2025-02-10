use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849936: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_936,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM C0NtRiVER protected",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4E, 0x54, 0x58, 0xE8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
