use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855253: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_253,
        source_type: SourceType::Wikidata,
        name: "AVG Antivirus Vault file (v7)",
        extensions: &["fil"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x56, 0x47, 0x37, 0x5F, 0x41, 0x4E, 0x54, 0x49, 0x56, 0x49, 0x52,
                        0x55, 0x53, 0x5F, 0x56, 0x41, 0x55, 0x4C, 0x54, 0x5F, 0x46, 0x49, 0x4C,
                        0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
