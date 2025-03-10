use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867010: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_010,
        source_type: SourceType::Wikidata,
        name: "Notepad++ session",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4E, 0x6F, 0x74, 0x65, 0x70, 0x61, 0x64, 0x50, 0x6C, 0x75, 0x73,
                        0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x3C, 0x53, 0x65, 0x73, 0x73,
                        0x69, 0x6F, 0x6E, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x56, 0x69,
                        0x65, 0x77, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
