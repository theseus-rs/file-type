use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851284: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_284,
        source_type: SourceType::Wikidata,
        name: "Tagwrite Template",
        extensions: &["tww"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x61, 0x67, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x54, 0x65, 0x6D,
                        0x70, 0x6C, 0x61, 0x74, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x31,
                        0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
