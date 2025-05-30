use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866990: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_990,
        source_type: SourceType::Wikidata,
        name: "Navigon generic data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x11, 0xFE, 0xD5, 0x0B, 0x05, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
