use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864905: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_905,
        source_type: SourceType::Wikidata,
        name: "PocketBook Theme",
        extensions: &["pbt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6F, 0x63, 0x6B, 0x65, 0x74, 0x42, 0x6F, 0x6F, 0x6B, 0x54, 0x68,
                        0x65, 0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
