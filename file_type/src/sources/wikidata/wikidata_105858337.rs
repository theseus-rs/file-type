use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858337: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_337,
        source_type: SourceType::Wikidata,
        name: "Easel.ly graphic project",
        extensions: &["ely"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x6A, 0x73, 0x6F, 0x6E, 0x22, 0x3A, 0x22, 0x7B, 0x5C, 0x22,
                        0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x73, 0x5C, 0x22, 0x3A, 0x5B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
