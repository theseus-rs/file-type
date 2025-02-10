use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864405: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_405,
        source_type: SourceType::Wikidata,
        name: "PHREEQC data",
        extensions: &["pqo"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x20, 0x20, 0x49, 0x6E, 0x70, 0x75, 0x74, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
