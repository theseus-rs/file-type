use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849890: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_890,
        source_type: SourceType::Wikidata,
        name: "MS-DOS International Code Page Info",
        extensions: &["cpi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x46, 0x4F, 0x4E, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
