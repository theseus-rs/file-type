use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856471: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_471,
        source_type: SourceType::Wikidata,
        name: "SoftKey WinWorks document Template",
        extensions: &["wpt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x6A, 0x00, 0x57, 0x49, 0x4E, 0x57, 0x4F, 0x52, 0x4B, 0x53,
                        0x20, 0x54, 0x45, 0x4D, 0x50, 0x4C, 0x41, 0x54, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
