use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857306: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_306,
        source_type: SourceType::Wikidata,
        name: "C-Worthy Help Librarian Data (v2.x)",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x2D, 0x57, 0x6F, 0x72, 0x74, 0x68, 0x79, 0x20, 0x28, 0x52, 0x29,
                        0x20, 0x48, 0x65, 0x6C, 0x70, 0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72,
                        0x69, 0x61, 0x6E, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C,
                        0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x32, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
