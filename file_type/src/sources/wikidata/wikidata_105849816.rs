use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849816: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_816,
        source_type: SourceType::Wikidata,
        name: "cFosSpeed registration key",
        extensions: &["cfosspeed"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x0D, 0x00, 0x0A, 0x00, 0x5B, 0x00, 0x6C, 0x00, 0x69, 0x00,
                        0x63, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x73, 0x00, 0x65, 0x00, 0x5D, 0x00,
                        0x0D, 0x00, 0x0A, 0x00, 0x6B, 0x00, 0x74, 0x00, 0x79, 0x00, 0x70, 0x00,
                        0x65, 0x00, 0x3D, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
