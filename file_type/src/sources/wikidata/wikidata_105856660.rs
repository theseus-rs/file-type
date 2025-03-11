use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856660: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_660,
        source_type: SourceType::Wikidata,
        name: "UltraCard Stack",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x54, 0x41, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
