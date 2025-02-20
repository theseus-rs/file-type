use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850973: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_973,
        source_type: SourceType::Wikidata,
        name: "GameCube THP video",
        extensions: &["thp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x48, 0x50, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
