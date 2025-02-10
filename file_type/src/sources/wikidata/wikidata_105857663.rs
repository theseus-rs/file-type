use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857663: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_663,
        source_type: SourceType::Wikidata,
        name: "The Duplicator Toolkit disk image",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x6F,
                        0x66, 0x20, 0x61, 0x20, 0x64, 0x69, 0x73, 0x6B, 0x65, 0x74, 0x74, 0x65,
                        0x20, 0x62, 0x79, 0x20, 0x54, 0x48, 0x45, 0x20, 0x44, 0x55, 0x50, 0x4C,
                        0x49, 0x43, 0x41, 0x54, 0x4F, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
