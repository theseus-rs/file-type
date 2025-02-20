use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_725,
        source_type: SourceType::Wikidata,
        name: "Twist Update script",
        extensions: &["u"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
