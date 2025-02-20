use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864492: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_492,
        source_type: SourceType::Wikidata,
        name: "Battlezone 2 game data package",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
