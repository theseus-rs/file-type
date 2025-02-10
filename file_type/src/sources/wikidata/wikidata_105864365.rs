use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864365: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_365,
        source_type: SourceType::Wikidata,
        name: "PCVIC VIC-20 emulator saved-session",
        extensions: &["pcv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x56, 0x49, 0x43, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6D,
                        0x20, 0x73, 0x6E, 0x61, 0x70, 0x73, 0x68, 0x6F, 0x74, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
