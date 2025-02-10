use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27960142: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_142,
        source_type: SourceType::Wikidata,
        name: "WVE",
        extensions: &["wve"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4C, 0x61, 0x77, 0x53, 0x6F, 0x75, 0x6E, 0x64, 0x46, 0x69, 0x6C,
                        0x65, 0x2A, 0x2A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
