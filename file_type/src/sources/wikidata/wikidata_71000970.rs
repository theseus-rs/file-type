use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_71000970: FileType = FileType {
    file_format: &FileFormat {
        id: 71_000_970,
        source_type: SourceType::Wikidata,
        name: "OmniGraffle Drawing",
        extensions: &["graffle"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                        0x3F, 0x3E, 0x0A, 0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45,
                        0x20, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
