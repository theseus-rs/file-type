use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867097: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_097,
        source_type: SourceType::Wikidata,
        name: "NetHack 3.4.1 saved game",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x04, 0x03, 0x46, 0x0C, 0x1E, 0x18, 0x7D, 0x01, 0x1B, 0x21,
                        0xCC, 0x95, 0x81, 0xA4,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
