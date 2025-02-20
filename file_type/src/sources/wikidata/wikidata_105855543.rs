use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855543: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_543,
        source_type: SourceType::Wikidata,
        name: "Psion Object/OPL Output",
        extensions: &["app", "opa", "opo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x50, 0x4C, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x46, 0x69, 0x6C,
                        0x65, 0x2A, 0x2A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
