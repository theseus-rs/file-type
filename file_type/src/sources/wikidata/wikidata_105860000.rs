use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860000: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_000,
        source_type: SourceType::Wikidata,
        name: "VBA32 Antivirus Signature",
        extensions: &["udb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x42, 0x41, 0x33, 0x32, 0x20, 0x56, 0x69, 0x72, 0x75, 0x73, 0x20,
                        0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x55, 0x70, 0x64,
                        0x61, 0x74, 0x65, 0x2C, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
