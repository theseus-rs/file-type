use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855923: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_923,
        source_type: SourceType::Wikidata,
        name: "XTreeGold graphics Driver",
        extensions: &["drv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x54, 0x47, 0x44, 0x52, 0x56, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
