use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862437: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_437,
        source_type: SourceType::Wikidata,
        name: "Great Valley Products EGS settings",
        extensions: &["map2video"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x50, 0x52, 0x46, 0x4D, 0x55, 0x53, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
