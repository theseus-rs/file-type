use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_609,
        source_type: SourceType::Wikidata,
        name: "ZX-Modules Meta format",
        extensions: &["zxb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x2D, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x73, 0x20, 0x4D,
                        0x65, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
