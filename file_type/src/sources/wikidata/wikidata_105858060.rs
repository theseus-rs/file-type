use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_060,
        source_type: SourceType::Wikidata,
        name: "iClone format (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x52, 0x4C, 0x5A, 0x65, 0x75, 0x73, 0x03, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x00, 0x00, 0x70, 0xB7, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x43,
                        0x54, 0x68, 0x75, 0x6D, 0x62, 0x6E, 0x61, 0x69, 0x6C, 0x5E, 0xB7, 0x00,
                        0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
