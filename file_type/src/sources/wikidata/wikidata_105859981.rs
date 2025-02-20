use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859981: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_981,
        source_type: SourceType::Wikidata,
        name: "Vice Snapshot File",
        extensions: &["vsf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x49, 0x43, 0x45, 0x20, 0x53, 0x6E, 0x61, 0x70, 0x73, 0x68, 0x6F,
                        0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
