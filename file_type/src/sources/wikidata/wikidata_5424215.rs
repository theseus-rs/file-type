use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5424215: FileType = FileType {
    file_format: &FileFormat {
        id: 5_424_215,
        source_type: SourceType::Wikidata,
        name: "F3",
        extensions: &["f3b"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x13, 0x7A, 0x29, 0x51, 0x00, 0x00, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
