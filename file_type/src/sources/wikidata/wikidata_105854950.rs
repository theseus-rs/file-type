use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854950: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_950,
        source_type: SourceType::Wikidata,
        name: "Artweaver Brush",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F, 0x3E, 0x0D, 0x0A, 0x3C,
                        0x21, 0x2D, 0x2D, 0x20, 0x41, 0x72, 0x74, 0x77, 0x65, 0x61, 0x76, 0x65,
                        0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
