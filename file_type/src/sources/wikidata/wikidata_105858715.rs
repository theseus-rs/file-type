use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858715: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_715,
        source_type: SourceType::Wikidata,
        name: "BootSkin Vista theme",
        extensions: &["bootskin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x53, 0x56, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
