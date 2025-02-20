use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_823,
        source_type: SourceType::Wikidata,
        name: "BluffTitler Show",
        extensions: &["bt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x42, 0x00, 0x6C, 0x00, 0x75, 0x00, 0x66, 0x00, 0x66, 0x00,
                        0x54, 0x00, 0x69, 0x00, 0x74, 0x00, 0x6C, 0x00, 0x65, 0x00, 0x72, 0x00,
                        0x20, 0x00, 0x53, 0x00, 0x68, 0x00, 0x6F, 0x00, 0x77, 0x00, 0x20, 0x00,
                        0x34, 0x00, 0x0D, 0x00, 0x0A, 0x00, 0x4C, 0x00, 0x45, 0x00, 0x4E, 0x00,
                        0x09, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
