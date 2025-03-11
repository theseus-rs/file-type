use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851371: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_371,
        source_type: SourceType::Wikidata,
        name: "TPW enctrypted data (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0D, 0x15, 0x22, 0x37, 0x59, 0x90,
                        0xE9, 0x90, 0x59, 0x37, 0x54, 0x50, 0x57, 0x20, 0x56, 0x65, 0x72, 0x20,
                        0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
