use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856997: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_997,
        source_type: SourceType::Wikidata,
        name: "MicroImages GPS Log (v1)",
        extensions: &["gps"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x73, 0x20,
                        0x47, 0x50, 0x53, 0x20, 0x4C, 0x6F, 0x67, 0x20, 0x56, 0x65, 0x72, 0x73,
                        0x69, 0x6F, 0x6E, 0x20, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
