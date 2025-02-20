use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858583: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_583,
        source_type: SourceType::Wikidata,
        name: "Gladius game data archive (PS2)",
        extensions: &["bec"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x63, 0x65, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
