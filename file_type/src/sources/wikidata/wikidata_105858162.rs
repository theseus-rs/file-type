use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858162: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_162,
        source_type: SourceType::Wikidata,
        name: "MegaPaint INF",
        extensions: &["inf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x49, 0x4E, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
