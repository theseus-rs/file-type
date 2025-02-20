use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855960: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_960,
        source_type: SourceType::Wikidata,
        name: "Studio Printer Dither method",
        extensions: &["dit"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x64, 0x69, 0x74,
                        0x68, 0x65, 0x72, 0x20, 0x6D, 0x65, 0x74, 0x68, 0x6F, 0x64, 0x3A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
