use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_308,
        source_type: SourceType::Wikidata,
        name: "Necromancer's Dos Navigator Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x63, 0x72, 0x6F, 0x6D, 0x61, 0x6E, 0x63, 0x65, 0x72, 0x27,
                        0x73, 0x20, 0x44, 0x6F, 0x73, 0x20, 0x4E, 0x61, 0x76, 0x69, 0x67, 0x61,
                        0x74, 0x6F, 0x72, 0x20, 0x48, 0x65, 0x6C, 0x70, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x2E, 0x2E, 0x2E, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
