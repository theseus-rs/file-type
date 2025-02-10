use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859032: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_032,
        source_type: SourceType::Wikidata,
        name: "Binary Point File 3",
        extensions: &["bpf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x50, 0x46, 0x21, 0x30, 0x30, 0x30, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
