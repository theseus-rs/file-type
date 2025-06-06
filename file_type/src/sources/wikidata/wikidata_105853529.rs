use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853529: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_529,
        source_type: SourceType::Wikidata,
        name: "Zelio Soft project",
        extensions: &["zel"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x27, 0x31, 0x2E, 0x30, 0x27, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x27, 0x77, 0x69, 0x6E, 0x64, 0x6F, 0x77,
                        0x73, 0x2D, 0x31, 0x32, 0x35, 0x32, 0x27, 0x20, 0x3F, 0x3E, 0x0D, 0x0A,
                        0x3C, 0x5A, 0x65, 0x6C, 0x69, 0x6F, 0x3E, 0x0D, 0x0A, 0x09, 0x3C, 0x46,
                        0x69, 0x6C, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E, 0x37,
                        0x3C, 0x2F, 0x46, 0x69, 0x6C, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3E, 0x0D, 0x0A, 0x09, 0x3C, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65,
                        0x5F, 0x49, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x74, 0x79, 0x3E, 0x0D, 0x0A,
                        0x09, 0x09, 0x3C, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65,
                        0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
