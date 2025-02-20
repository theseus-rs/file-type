use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_384,
        source_type: SourceType::Wikidata,
        name: "Ren'Py Compiled game",
        extensions: &["rpyc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x4E, 0x50, 0x59, 0x20, 0x52, 0x50, 0x43, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
