use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861375: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_375,
        source_type: SourceType::Wikidata,
        name: "Linux Software Map entry (v4)",
        extensions: &["lsm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x65, 0x67, 0x69, 0x6E, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
