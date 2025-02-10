use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864884: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_884,
        source_type: SourceType::Wikidata,
        name: "Starbound player data (v1)",
        extensions: &["player"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x50, 0x46, 0x56, 0x31, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
