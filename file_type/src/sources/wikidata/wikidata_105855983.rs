use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855983: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_983,
        source_type: SourceType::Wikidata,
        name: "Deep Zoom Composer Project",
        extensions: &["dzprj"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F, 0x3E, 0x0D, 0x0A, 0x3C,
                        0x44, 0x65, 0x65, 0x70, 0x5A, 0x6F, 0x6F, 0x6D, 0x43, 0x6F, 0x6D, 0x70,
                        0x6F, 0x73, 0x65, 0x72, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
