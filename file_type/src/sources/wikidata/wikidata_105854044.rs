use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854044: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_044,
        source_type: SourceType::Wikidata,
        name: "Elcomsoft ADC Advanced Disc Catalog",
        extensions: &["cat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0x00, 0x00, 0x00, 0x41, 0x44, 0x43, 0x2E, 0x43, 0x61, 0x74, 0x61,
                        0x6C, 0x6F, 0x67,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
