use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861010: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_010,
        source_type: SourceType::Wikidata,
        name: "Opticks Lamp",
        extensions: &["lmp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x4C, 0x41, 0x4D, 0x50, 0x76, 0x31, 0x2E, 0x30, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
