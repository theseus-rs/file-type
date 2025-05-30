use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864216: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_216,
        source_type: SourceType::Wikidata,
        name: "Professional Music Driver P86 samples pack",
        extensions: &["p86"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x4D, 0x38, 0x36, 0x20, 0x44, 0x41, 0x54, 0x41, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
