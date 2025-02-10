use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_100235503: FileType = FileType {
    file_format: &FileFormat {
        id: 100_235_503,
        source_type: SourceType::Wikidata,
        name: "FinePrint file format",
        extensions: &["fp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x49, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
