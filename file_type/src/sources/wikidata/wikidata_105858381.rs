use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858381: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_381,
        source_type: SourceType::Wikidata,
        name: "E-Mail message (Var. 7)",
        extensions: &["eml"],
        media_types: &["message/rfc822"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x65, 0x6C, 0x69, 0x76, 0x65, 0x72, 0x65, 0x64, 0x2D, 0x54, 0x6F,
                        0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
