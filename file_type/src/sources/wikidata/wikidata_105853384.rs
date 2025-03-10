use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_384,
        source_type: SourceType::Wikidata,
        name: "Storm Shell project/makefile",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x74, 0x6F, 0x72, 0x6D, 0x20, 0x53, 0x68, 0x65, 0x6C, 0x6C, 0x20,
                        0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x28, 0x30, 0x30, 0x31,
                        0x33, 0x29, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
