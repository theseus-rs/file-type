use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851426: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_426,
        source_type: SourceType::Wikidata,
        name: "Turboprint color info (v4)",
        extensions: &["tpm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x54, 0x75, 0x72, 0x62, 0x6F, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x20,
                        0x34, 0x2E, 0x30, 0x20, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x5D, 0x0A, 0x0A,
                        0x6E, 0x75, 0x6D, 0x74, 0x61, 0x62, 0x6C, 0x65, 0x73, 0x0A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
