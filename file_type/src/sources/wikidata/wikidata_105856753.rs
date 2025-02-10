use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856753: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_753,
        source_type: SourceType::Wikidata,
        name: "UC Browser Theme",
        extensions: &["uct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0xFC, 0xBC, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
