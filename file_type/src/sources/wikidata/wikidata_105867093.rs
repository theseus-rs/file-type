use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_093,
        source_type: SourceType::Wikidata,
        name: "NOD32 module",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x4F, 0x44, 0x33, 0x32, 0x2D, 0x4D, 0x4F, 0x44, 0x55, 0x4C, 0x45,
                        0x20, 0x2D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
