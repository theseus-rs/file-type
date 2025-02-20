use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71264752: FileType = FileType {
    file_format: &FileFormat {
        id: 71_264_752,
        source_type: SourceType::Wikidata,
        name: "Hippel 7V module",
        extensions: &["hip7"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x60, 0x00, 0x00, 0x2A, 0x20, 0x2A, 0x2A, 0x2A, 0x2A, 0x20, 0x50, 0x6C,
                        0x61, 0x79, 0x65, 0x72, 0x20, 0x62, 0x79, 0x20, 0x4A, 0x6F, 0x63, 0x68,
                        0x65, 0x6E, 0x20, 0x48, 0x69, 0x70, 0x70, 0x65, 0x6C, 0x20, 0x31, 0x39,
                        0x39, 0x30, 0x20, 0x2A, 0x2A, 0x2A, 0x2A, 0x20, 0x48, 0xE7, 0xFF, 0xFE,
                        0x4A, 0x41, 0x6B, 0x08, 0x67, 0x06, 0x41, 0xFA, 0x04,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
