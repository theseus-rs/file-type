use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28804256: FileType = FileType {
    file_format: &FileFormat {
        id: 28_804_256,
        source_type: SourceType::Wikidata,
        name: "Uniform Office Presentation",
        extensions: &["uop"],
        media_types: &["application/vnd.uof.presentation"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                        0x20, 0x73, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x6C, 0x6F, 0x6E, 0x65, 0x3D,
                        0x22, 0x6E, 0x6F, 0x22, 0x3F, 0x3E, 0x0A, 0x3C, 0x75, 0x6F, 0x66, 0x3A,
                        0x55, 0x4F, 0x46, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x75, 0x6F,
                        0x66, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x73, 0x63,
                        0x68, 0x65, 0x6D, 0x61, 0x73, 0x2E, 0x75, 0x6F, 0x66, 0x2E, 0x6F, 0x72,
                        0x67, 0x2F, 0x63, 0x6E, 0x2F, 0x32, 0x30, 0x30, 0x33, 0x2F, 0x75, 0x6F,
                        0x66, 0x22, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0xE5, 0x9B, 0xBE,
                        0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x73, 0x63, 0x68,
                        0x65, 0x6D, 0x61, 0x73, 0x2E, 0x75, 0x6F, 0x66, 0x2E, 0x6F, 0x72, 0x67,
                        0x2F, 0x63, 0x6E, 0x2F, 0x32, 0x30, 0x30, 0x33, 0x2F, 0x67, 0x72, 0x61,
                        0x70, 0x68, 0x22, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0xE5, 0xAD,
                        0x97, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x73, 0x63,
                        0x68, 0x65, 0x6D, 0x61, 0x73, 0x2E, 0x75, 0x6F, 0x66, 0x2E, 0x6F, 0x72,
                        0x67, 0x2F, 0x63, 0x6E, 0x2F, 0x32, 0x30, 0x30, 0x33, 0x2F, 0x75, 0x6F,
                        0x66, 0x2D, 0x77, 0x6F, 0x72, 0x64, 0x70, 0x72, 0x6F, 0x63, 0x22, 0x20,
                        0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0xE6, 0xBC, 0x94, 0x3D, 0x22, 0x68,
                        0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61,
                        0x73, 0x2E, 0x75, 0x6F, 0x66, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x63, 0x6E,
                        0x2F, 0x32, 0x30, 0x30, 0x33, 0x2F, 0x75, 0x6F, 0x66, 0x2D, 0x73, 0x6C,
                        0x69, 0x64, 0x65, 0x73, 0x68, 0x6F, 0x77, 0x22, 0x20, 0x78, 0x6D, 0x6C,
                        0x6E, 0x73, 0x3A, 0xE8, 0xA1, 0xA8, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70,
                        0x3A, 0x2F, 0x2F, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x73, 0x2E, 0x75,
                        0x6F, 0x66, 0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x63, 0x6E, 0x2F, 0x32, 0x30,
                        0x30, 0x33, 0x2F, 0x75, 0x6F, 0x66, 0x2D, 0x73, 0x70, 0x72, 0x65, 0x61,
                        0x64, 0x73, 0x68, 0x65, 0x65, 0x74, 0x22, 0x20, 0x78, 0x6D, 0x6C, 0x6E,
                        0x73, 0x3A, 0x78, 0x73, 0x69, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A,
                        0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x77, 0x33, 0x2E, 0x6F, 0x72, 0x67,
                        0x2F, 0x32, 0x30, 0x30, 0x31, 0x2F, 0x58, 0x4D, 0x4C, 0x53, 0x63, 0x68,
                        0x65, 0x6D, 0x61, 0x2D, 0x69, 0x6E, 0x73, 0x74, 0x61, 0x6E, 0x63, 0x65,
                        0x22, 0x20, 0x75, 0x6F, 0x66, 0x3A, 0x6C, 0x61, 0x6E, 0x67, 0x75, 0x61,
                        0x67, 0x65, 0x3D, 0x22, 0x63, 0x6E, 0x22, 0x20, 0x75, 0x6F, 0x66, 0x3A,
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30,
                        0x22, 0x20, 0x75, 0x6F, 0x66, 0x3A, 0x6C, 0x6F, 0x63, 0x49, 0x44, 0x3D,
                        0x22, 0x75, 0x30, 0x30, 0x30, 0x30, 0x22, 0x20, 0x75, 0x6F, 0x66, 0x3A,
                        0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x3D, 0x22, 0x76, 0x6E,
                        0x64, 0x2E, 0x75, 0x6F, 0x66, 0x2E, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6E,
                        0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x22, 0x3E, 0x3C, 0x75, 0x6F, 0x66,
                        0x3A, 0xE5, 0x85, 0x83, 0xE6, 0x95, 0xB0, 0xE6, 0x8D, 0xAE, 0x20, 0x75,
                        0x6F, 0x66, 0x3A, 0x6C, 0x6F, 0x63, 0x49, 0x44, 0x3D, 0x22, 0x75, 0x30,
                        0x30, 0x30, 0x31, 0x22, 0x3E, 0x3C, 0x75, 0x6F, 0x66, 0x3A, 0xE6, 0xA0,
                        0x87, 0xE9, 0xA2, 0x98, 0x20, 0x75, 0x6F, 0x66, 0x3A, 0x6C, 0x6F, 0x63,
                        0x49, 0x44, 0x3D, 0x22, 0x75, 0x30, 0x30, 0x30, 0x32, 0x22, 0x2F, 0x3E,
                        0x3C, 0x75, 0x6F, 0x66, 0x3A, 0xE5, 0x88, 0x9B, 0xE5, 0xBB, 0xBA, 0xE5,
                        0xBA, 0x94, 0xE7, 0x94, 0xA8, 0xE7, 0xA8, 0x8B, 0xE5, 0xBA, 0x8F, 0x20,
                        0x75, 0x6F, 0x66, 0x3A, 0x6C, 0x6F, 0x63, 0x49, 0x44, 0x3D, 0x22, 0x75,
                        0x30, 0x30, 0x31, 0x31, 0x22, 0x3E, 0x4C, 0x69, 0x62, 0x72, 0x65, 0x4F,
                        0x66, 0x66, 0x69, 0x63, 0x65, 0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
