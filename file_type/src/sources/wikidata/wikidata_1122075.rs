use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_1122075: FileType = FileType {
    file_format: &FileFormat {
        id: 1_122_075,
        source_type: SourceType::Wikidata,
        name: "Computable Document Format",
        extensions: &["cdf"],
        media_types: &["application/cdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x2A, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x74,
                        0x79, 0x70, 0x65, 0x3A, 0x20, 0x61, 0x70, 0x70, 0x6C, 0x69, 0x63, 0x61,
                        0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x76, 0x6E, 0x64, 0x2E, 0x77, 0x6F, 0x6C,
                        0x66, 0x72, 0x61, 0x6D, 0x2E, 0x63, 0x64, 0x66, 0x2E, 0x74, 0x65, 0x78,
                        0x74, 0x20, 0x2A, 0x29,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
