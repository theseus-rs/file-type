use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_142,
        source_type: SourceType::Wikidata,
        name: "NASCA DRM encrypted",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x23, 0x23, 0x20, 0x4E, 0x41, 0x53, 0x43, 0x41, 0x20, 0x44, 0x52,
                        0x4D, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x2D, 0x20, 0x56, 0x45, 0x52,
                        0x31, 0x2E, 0x30, 0x30, 0x20, 0x23, 0x23, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
