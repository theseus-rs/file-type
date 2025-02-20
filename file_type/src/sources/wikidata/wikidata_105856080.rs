use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_080,
        source_type: SourceType::Wikidata,
        name: "CHEMVIEW animation Data",
        extensions: &["d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x74, 0x6F, 0x6D, 0x6C, 0x6F, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        0x28,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
