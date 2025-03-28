use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858545: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_545,
        source_type: SourceType::Wikidata,
        name: "CHDK UBASIC script",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
