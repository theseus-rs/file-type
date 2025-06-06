use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850313: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_313,
        source_type: SourceType::Wikidata,
        name: "Crick Software Clicker File",
        extensions: &["clkx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65,
                        0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x75, 0x74, 0x66,
                        0x2D, 0x38, 0x22, 0x3F, 0x3E, 0x0D, 0x0A, 0x3C, 0x63, 0x72, 0x69, 0x63,
                        0x6B, 0x67, 0x72, 0x69, 0x64, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x3C, 0x63,
                        0x69, 0x3A, 0x63, 0x72, 0x69, 0x63, 0x6B, 0x69, 0x6E, 0x66, 0x6F, 0x20,
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x22, 0x20,
                        0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x63, 0x69, 0x3D, 0x22, 0x68, 0x74,
                        0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x63, 0x72, 0x69,
                        0x63, 0x6B, 0x73, 0x6F, 0x66, 0x74, 0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x73,
                        0x63, 0x68, 0x65, 0x6D, 0x61, 0x73, 0x2F, 0x66, 0x69, 0x6C, 0x65, 0x2F,
                        0x63, 0x72, 0x69, 0x63, 0x6B, 0x69, 0x6E, 0x66, 0x6F, 0x2E, 0x78, 0x73,
                        0x64, 0x22, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x3C, 0x63, 0x69,
                        0x3A, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x61, 0x70, 0x70, 0x20, 0x6E,
                        0x61, 0x6D, 0x65, 0x3D, 0x22, 0x43, 0x6C, 0x69, 0x63, 0x6B, 0x65, 0x72,
                        0x22, 0x20, 0x2F, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x3C, 0x63,
                        0x69, 0x3A, 0x61, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x61, 0x70, 0x70, 0x20,
                        0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x22, 0x43, 0x6C, 0x69, 0x63, 0x6B, 0x65,
                        0x72, 0x20, 0x35, 0x22, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
