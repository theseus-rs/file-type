use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865697: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_697,
        source_type: SourceType::Wikidata,
        name: "Messenger Plus! Encrypted chat log",
        extensions: &["ple"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x10, 0x01, 0x4D, 0x50, 0x4C, 0x45, 0x31, 0x3C, 0x3C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
