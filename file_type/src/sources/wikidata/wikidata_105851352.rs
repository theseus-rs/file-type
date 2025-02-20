use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851352: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_352,
        source_type: SourceType::Wikidata,
        name: "Cura theme",
        extensions: &["json"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x22, 0x6D, 0x65, 0x74, 0x61, 0x64,
                        0x61, 0x74, 0x61, 0x22, 0x3A, 0x20, 0x7B, 0x0A, 0x20, 0x20, 0x20, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x22, 0x6E, 0x61, 0x6D, 0x65, 0x22, 0x3A, 0x20,
                        0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
