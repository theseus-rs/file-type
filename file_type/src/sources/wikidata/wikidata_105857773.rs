use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857773: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_773,
        source_type: SourceType::Wikidata,
        name: "STK Interval List format",
        extensions: &["int"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x74, 0x6B, 0x2E, 0x76, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
