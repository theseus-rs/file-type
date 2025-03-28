use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861033: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_033,
        source_type: SourceType::Wikidata,
        name: "Lexar Encrypted file",
        extensions: &["lrs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x45, 0x58, 0x41, 0x52, 0x45, 0x4E, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
