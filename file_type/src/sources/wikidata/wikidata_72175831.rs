use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_72175831: FileType = FileType {
    file_format: &FileFormat {
        id: 72_175_831,
        source_type: SourceType::Wikidata,
        name: "KDELNK",
        extensions: &["KDELNK"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x4B, 0x44, 0x45, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67,
                        0x20, 0x46, 0x69, 0x6C, 0x65, 0x0A, 0x5B, 0x4B, 0x44, 0x45, 0x20, 0x44,
                        0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x20, 0x45, 0x6E, 0x74, 0x72, 0x79,
                        0x5D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
