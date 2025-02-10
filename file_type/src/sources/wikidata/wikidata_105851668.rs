use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851668: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_668,
        source_type: SourceType::Wikidata,
        name: "Sublime Text Snippets",
        extensions: &["sublime-snippet"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x73, 0x6E, 0x69, 0x70, 0x70, 0x65, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
