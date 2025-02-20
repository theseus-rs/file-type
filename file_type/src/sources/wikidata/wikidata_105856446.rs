use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856446: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_446,
        source_type: SourceType::Wikidata,
        name: "Windows Sidebar Style",
        extensions: &["wsstyles"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x52, 0x53, 0x54, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
