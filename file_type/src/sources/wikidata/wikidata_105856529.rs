use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856529: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_529,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works wizard",
        extensions: &["wpj"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C, 0x00,
                        0x20, 0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00,
                        0x6F, 0x00, 0x6E, 0x00, 0x3D, 0x00, 0x22, 0x00, 0x31, 0x00, 0x2E, 0x00,
                        0x30, 0x00, 0x22, 0x00, 0x20, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x63, 0x00,
                        0x6F, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x67, 0x00, 0x3D, 0x00,
                        0x22, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
