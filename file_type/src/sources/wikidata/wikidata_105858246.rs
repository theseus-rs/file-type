use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858246: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_246,
        source_type: SourceType::Wikidata,
        name: "Executor Configuration File",
        extensions: &["ecf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x43, 0x6F, 0x6E, 0x66,
                        0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69,
                        0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
