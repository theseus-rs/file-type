use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855311: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_311,
        source_type: SourceType::Wikidata,
        name: "Furcadia Art",
        extensions: &["fox"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x53, 0x48, 0x58, 0x01, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00,
                        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
