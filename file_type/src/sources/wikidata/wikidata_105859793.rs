use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859793: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_793,
        source_type: SourceType::Wikidata,
        name: "Vizact 2000 Wizard",
        extensions: &["vzw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x56, 0x49, 0x5A, 0x41, 0x43, 0x54, 0x57, 0x49, 0x5A, 0x41, 0x52,
                        0x44, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3D, 0x22, 0x31,
                        0x2E, 0x30, 0x22, 0x3E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
