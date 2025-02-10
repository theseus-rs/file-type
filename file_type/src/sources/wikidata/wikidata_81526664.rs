use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_81526664: FileType = FileType {
    file_format: &FileFormat {
        id: 81_526_664,
        source_type: SourceType::Wikidata,
        name: "Palm Desktop DateBook",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xBE, 0xBA, 0xFE, 0xCA, 0x0F, 0x50, 0x61, 0x6C, 0x6D, 0x53, 0x47, 0x20,
                        0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
