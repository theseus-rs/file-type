use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854869: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_869,
        source_type: SourceType::Wikidata,
        name: "a-squared Anti-Malware signature",
        extensions: &["sig"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4D, 0x53, 0x49, 0x88, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
