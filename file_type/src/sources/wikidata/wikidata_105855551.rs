use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855551: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_551,
        source_type: SourceType::Wikidata,
        name: "OOMMF Vector Field 2.0 format",
        extensions: &["ovf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x4F, 0x4F, 0x4D, 0x4D, 0x46, 0x20, 0x4F, 0x56, 0x46, 0x20,
                        0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
