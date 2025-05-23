use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_093,
        source_type: SourceType::Wikidata,
        name: "Wise Installer log",
        extensions: &["log"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x2A, 0x2A, 0x20, 0x20, 0x49, 0x6E, 0x73, 0x74, 0x61, 0x6C, 0x6C,
                        0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65,
                        0x64, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
