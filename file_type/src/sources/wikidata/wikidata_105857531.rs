use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857531: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_531,
        source_type: SourceType::Wikidata,
        name: "Parsons Technology resource Index",
        extensions: &["idx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x52, 0x53, 0x4F, 0x4E, 0x53, 0x20, 0x54, 0x45, 0x43, 0x48,
                        0x4E, 0x4F, 0x4C, 0x4F, 0x47, 0x59, 0x20, 0x52, 0x45, 0x53, 0x4F, 0x55,
                        0x52, 0x43, 0x45, 0x20, 0x49, 0x4E, 0x44, 0x45, 0x58, 0x20, 0x46, 0x49,
                        0x4C, 0x45, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
