use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859511: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_511,
        source_type: SourceType::Wikidata,
        name: "TiVo video",
        extensions: &["ty"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF5, 0x46, 0x7A, 0xBD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
