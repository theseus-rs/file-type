use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857007: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_007,
        source_type: SourceType::Wikidata,
        name: "Mini Office Graph data",
        extensions: &["gra"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x4E, 0x49, 0x20, 0x4F, 0x46, 0x46, 0x49, 0x43, 0x45, 0x20,
                        0x41, 0x4D, 0x49, 0x47, 0x41, 0x20, 0x2D, 0x20, 0x47, 0x52, 0x41, 0x50,
                        0x48, 0x49, 0x43, 0x53, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20, 0x46, 0x49,
                        0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
