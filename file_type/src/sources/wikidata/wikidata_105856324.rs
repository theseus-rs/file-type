use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856324: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_324,
        source_type: SourceType::Wikidata,
        name: "SuperJAM! Drum map",
        extensions: &["drummap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x54, 0x4D, 0x50, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
