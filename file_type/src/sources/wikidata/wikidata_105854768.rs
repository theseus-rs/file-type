use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854768: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_768,
        source_type: SourceType::Wikidata,
        name: "AWL programming language (Var. 1)",
        extensions: &["awl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x52, 0x47, 0x41, 0x4E, 0x49, 0x5A, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                        0x5F, 0x42, 0x4C, 0x4F, 0x43, 0x4B, 0x20, 0x50, 0x52, 0x49, 0x4E, 0x43,
                        0x49, 0x50, 0x41, 0x4C, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
