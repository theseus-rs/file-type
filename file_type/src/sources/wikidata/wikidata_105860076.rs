use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_076,
        source_type: SourceType::Wikidata,
        name: "ParaView VTK Structured grid",
        extensions: &["vts"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F, 0x3E, 0x0A, 0x3C, 0x56,
                        0x54, 0x4B, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x3D,
                        0x22, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x47,
                        0x72, 0x69, 0x64, 0x22, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
