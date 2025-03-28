use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854951: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_951,
        source_type: SourceType::Wikidata,
        name: "XCR archive",
        extensions: &["xcr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x78, 0x63, 0x72, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x31, 0x2E, 0x30,
                        0x30, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
