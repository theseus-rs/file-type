use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858360: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_360,
        source_type: SourceType::Wikidata,
        name: "E-Mail message (Var. 6)",
        extensions: &["eml"],
        media_types: &["message/rfc822"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x54, 0x79, 0x70, 0x65,
                        0x3A, 0x20, 0x6D, 0x75, 0x6C, 0x74, 0x69, 0x70, 0x61, 0x72, 0x74, 0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
