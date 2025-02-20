use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855244: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_244,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Deluxe Font",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x44, 0x45, 0x4C, 0x55, 0x58, 0x45, 0x2E, 0x46, 0x4E, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
