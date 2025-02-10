use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856879: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_879,
        source_type: SourceType::Wikidata,
        name: "Microsoft Math worksheet",
        extensions: &["gcw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x43, 0x61, 0x73, 0x44, 0x6F, 0x63, 0x75, 0x6D,
                        0x65, 0x6E, 0x74, 0x20, 0x44, 0x6F, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x22, 0x32, 0x2E, 0x30, 0x22, 0x3E, 0x3C, 0x45, 0x6E,
                        0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x5A, 0x6F, 0x6F, 0x6D, 0x46, 0x61,
                        0x63, 0x74, 0x6F, 0x72, 0x3D, 0x22, 0x31, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
