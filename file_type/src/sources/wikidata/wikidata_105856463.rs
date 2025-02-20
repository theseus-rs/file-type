use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856463: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_463,
        source_type: SourceType::Wikidata,
        name: "WillMaker 5 project",
        extensions: &["ww5"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x4D, 0x4B, 0x52, 0x35, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
