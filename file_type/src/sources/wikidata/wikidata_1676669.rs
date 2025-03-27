use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1676669: FileType = FileType {
    file_format: &FileFormat {
        id: 1_676_669,
        source_type: SourceType::Wikidata,
        name: "JPEG File Interchange Format, version 1.02",
        extensions: &["jfi", "jfif", "jpe", "jpeg", "jpg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xD8, 0xFF, 0xE0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
