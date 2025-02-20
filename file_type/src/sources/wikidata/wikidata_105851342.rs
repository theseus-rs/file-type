use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851342: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_342,
        source_type: SourceType::Wikidata,
        name: "Duxbury conversion Table",
        extensions: &["tbl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4E, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x54,
                        0x41, 0x42, 0x4C, 0x45, 0x2E, 0x0D, 0x0A, 0x43, 0x2F, 0x4D, 0x20, 0x3A,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
