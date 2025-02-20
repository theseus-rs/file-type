use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_093,
        source_type: SourceType::Wikidata,
        name: "AVG 6 Integrity Database",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x56, 0x47, 0x36, 0x5F, 0x49, 0x6E, 0x74, 0x65, 0x67, 0x72, 0x69,
                        0x74, 0x79, 0x5F, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
