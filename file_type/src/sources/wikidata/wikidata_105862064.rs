use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862064: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_064,
        source_type: SourceType::Wikidata,
        name: "MicroMag 2900/3900 data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x4D, 0x61, 0x67, 0x20, 0x32, 0x39, 0x30,
                        0x30, 0x2F, 0x33, 0x39, 0x30, 0x30, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20,
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73,
                        0x20, 0x30, 0x30, 0x31, 0x35, 0x29, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
