use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859576: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_576,
        source_type: SourceType::Wikidata,
        name: "Visual Studio J# Project (v7)",
        extensions: &["vjsproj"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x56, 0x69, 0x73, 0x75, 0x61, 0x6C, 0x53, 0x74, 0x75, 0x64, 0x69,
                        0x6F, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
