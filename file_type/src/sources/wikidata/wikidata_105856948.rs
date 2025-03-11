use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856948: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_948,
        source_type: SourceType::Wikidata,
        name: "Game Maker data (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x91, 0xD5, 0x12, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
