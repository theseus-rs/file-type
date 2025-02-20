use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850686: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_686,
        source_type: SourceType::Wikidata,
        name: "Kv design language",
        extensions: &["kv"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x3A, 0x6B, 0x69, 0x76, 0x79, 0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
