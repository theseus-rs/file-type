use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856758: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_758,
        source_type: SourceType::Wikidata,
        name: "U-Boot uImage",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x27, 0x05, 0x19, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
