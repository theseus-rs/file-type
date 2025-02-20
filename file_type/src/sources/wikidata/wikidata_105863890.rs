use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863890: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_890,
        source_type: SourceType::Wikidata,
        name: "NetWare Message",
        extensions: &["msg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x74, 0x57, 0x61, 0x72, 0x65, 0x20, 0x4D, 0x65, 0x73, 0x73,
                        0x61, 0x67, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
