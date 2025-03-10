use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_384,
        source_type: SourceType::Wikidata,
        name: "ScriptBasic compiled binary",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x34, 0x42, 0x41, 0x53, 0x1A, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00,
                        0x00, 0x39, 0x73, 0x33, 0x33, 0x53, 0x54, 0x41, 0x4E, 0x44, 0x41, 0x52,
                        0x44, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
