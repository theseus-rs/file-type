use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861583: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_583,
        source_type: SourceType::Wikidata,
        name: "Paradox Lock",
        extensions: &["lck"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x61, 0x20, 0x6E, 0x65, 0x77, 0x65, 0x72, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
