use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857436: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_436,
        source_type: SourceType::Wikidata,
        name: "3DMark database",
        extensions: &["3db"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x33, 0x44, 0x4D, 0x61, 0x72, 0x6B, 0x20, 0x44, 0x61, 0x74, 0x61, 0x62,
                        0x61, 0x73, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
