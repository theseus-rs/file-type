use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867403: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_403,
        source_type: SourceType::Wikidata,
        name: "NeoplePack format",
        extensions: &["npk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x6F, 0x70, 0x6C, 0x65, 0x50, 0x61, 0x63, 0x6B, 0x5F, 0x42,
                        0x69, 0x6C, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
