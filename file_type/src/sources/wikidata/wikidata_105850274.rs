use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850274: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_274,
        source_type: SourceType::Wikidata,
        name: "16bit COM executable BAT2EXEC v1.5",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEB, 0x3A, 0x90, 0x43, 0x6F, 0x6D, 0x70, 0x69, 0x6C, 0x65, 0x64, 0x20,
                        0x62, 0x79, 0x20, 0x42, 0x41, 0x54, 0x32, 0x45, 0x58, 0x45, 0x43, 0x20,
                        0x31, 0x2E, 0x35, 0x0D, 0x0A, 0x50, 0x43, 0x20, 0x4D, 0x61, 0x67, 0x61,
                        0x7A, 0x69, 0x6E, 0x65, 0x20, 0xFE, 0x20, 0x44, 0x6F, 0x75, 0x67, 0x6C,
                        0x61, 0x73, 0x20, 0x42, 0x6F, 0x6C, 0x69, 0x6E, 0x67, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
