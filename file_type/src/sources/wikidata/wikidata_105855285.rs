use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855285: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_285,
        source_type: SourceType::Wikidata,
        name: "FastRay surface/material",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x76, 0x46, 0x52, 0x41, 0x59,
                        0x4F, 0x42, 0x45, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
