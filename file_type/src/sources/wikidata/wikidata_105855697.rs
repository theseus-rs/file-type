use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855697: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_697,
        source_type: SourceType::Wikidata,
        name: "Origin Pack game data archive",
        extensions: &["opk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x50, 0x4B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
