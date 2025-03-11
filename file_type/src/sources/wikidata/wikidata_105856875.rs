use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856875: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_875,
        source_type: SourceType::Wikidata,
        name: "Gizmoz Rollodex data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x02, 0x52, 0x4F, 0x4C, 0x4C, 0x44, 0x58,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
