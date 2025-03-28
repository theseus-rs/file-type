use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851892: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_892,
        source_type: SourceType::Wikidata,
        name: "Assault Wing Sprite",
        extensions: &["spr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x52, 0x50, 0x53, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
