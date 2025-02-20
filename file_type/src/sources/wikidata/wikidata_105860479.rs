use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860479: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_479,
        source_type: SourceType::Wikidata,
        name: "Eureka/Mercury Report",
        extensions: &["rpt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCD, 0xCD, 0xCD, 0xCD, 0xCD, 0x20, 0x50, 0x72, 0x6F, 0x62, 0x6C, 0x65,
                        0x6D, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
