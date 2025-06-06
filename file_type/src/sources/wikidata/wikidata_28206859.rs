use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206859: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_859,
        source_type: SourceType::Wikidata,
        name: "Polyomino Compressed Image Format",
        extensions: &["pcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x46, 0x2C, 0x20, 0x50, 0x6F, 0x6C, 0x79, 0x6F, 0x6D, 0x69,
                        0x6E, 0x6F, 0x20, 0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65,
                        0x64, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2E, 0x20, 0x41, 0x75,
                        0x74, 0x68, 0x6F, 0x72, 0x20, 0x53, 0x74, 0x65, 0x66, 0x61, 0x6E, 0x6F,
                        0x20, 0x42, 0x72, 0x6F, 0x63, 0x63, 0x68, 0x69, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
