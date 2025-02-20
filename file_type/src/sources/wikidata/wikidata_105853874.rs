use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853874: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_874,
        source_type: SourceType::Wikidata,
        name: "AWL programming language (Var. 2)",
        extensions: &["awl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x54, 0x41, 0x5F, 0x42, 0x4C, 0x4F, 0x43, 0x4B, 0x20, 0x44,
                        0x42, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
