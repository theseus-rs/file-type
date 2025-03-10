use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859171: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_171,
        source_type: SourceType::Wikidata,
        name: "BRS/Search dump",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x2A, 0x2A, 0x20, 0x42, 0x52, 0x53, 0x20, 0x44, 0x4F, 0x43, 0x55,
                        0x4D, 0x45, 0x4E, 0x54, 0x20, 0x42, 0x4F, 0x55, 0x4E, 0x44, 0x41, 0x52,
                        0x59, 0x20, 0x2A, 0x2A, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
