use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762763: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_763,
        source_type: SourceType::Wikidata,
        name: "Android Package with OBB data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x08, 0x00, 0x00, 0x00, 0x40, 0x46,
                        0x98, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
