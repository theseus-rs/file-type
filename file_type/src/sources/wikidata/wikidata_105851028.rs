use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851028: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_028,
        source_type: SourceType::Wikidata,
        name: "Turbo Assembler Help",
        extensions: &["tah"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x55, 0x52, 0x42, 0x4F, 0x20, 0x41, 0x53, 0x53, 0x45, 0x4D, 0x42,
                        0x4C, 0x45, 0x52, 0x20, 0x48, 0x45, 0x4C, 0x50, 0x20, 0x46, 0x49, 0x4C,
                        0x45, 0x2E, 0x00, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
