use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_735,
        source_type: SourceType::Wikidata,
        name: "Aksharamala Keymap Binary",
        extensions: &["akm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x31, 0x2E, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
