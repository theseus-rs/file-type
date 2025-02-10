use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_123138514: FileType = FileType {
    file_format: &FileFormat {
        id: 123_138_514,
        source_type: SourceType::Wikidata,
        name: "Disktracker Document",
        extensions: &["dtc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x69, 0x73, 0x6B, 0x74, 0x72, 0x61, 0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
