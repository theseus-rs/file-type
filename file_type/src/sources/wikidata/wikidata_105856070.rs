use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856070: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_070,
        source_type: SourceType::Wikidata,
        name: "DIGIBooster module",
        extensions: &["db", "digi"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x49, 0x47, 0x49, 0x20, 0x42, 0x6F, 0x6F, 0x73, 0x74, 0x65, 0x72,
                        0x20, 0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
