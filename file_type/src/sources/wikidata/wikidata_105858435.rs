use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858435: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_435,
        source_type: SourceType::Wikidata,
        name: "Bitmapped Signum!2 printer font (screen)",
        extensions: &["e24"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x65, 0x73, 0x65, 0x74, 0x30, 0x30, 0x30, 0x31, 0x00, 0x00, 0x00, 0x80,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
