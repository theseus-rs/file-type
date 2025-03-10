use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864855: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_855,
        source_type: SourceType::Wikidata,
        name: "Pack-Ice compressed data (new)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x43, 0x45, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
