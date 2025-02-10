use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861079: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_079,
        source_type: SourceType::Wikidata,
        name: "LabVIEW Local Project Settings",
        extensions: &["lvlps"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x57, 0x69, 0x6E, 0x64,
                        0x6F, 0x77, 0x5F, 0x44, 0x61, 0x74, 0x61, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
