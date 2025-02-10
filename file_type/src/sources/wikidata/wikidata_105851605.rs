use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851605: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_605,
        source_type: SourceType::Wikidata,
        name: "Adventure Game Studio Translation data",
        extensions: &["tra"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x47, 0x53, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x6C, 0x61, 0x74, 0x69,
                        0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
