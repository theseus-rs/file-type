use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858198: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_198,
        source_type: SourceType::Wikidata,
        name: "E-Mail message (Var. 10)",
        extensions: &["eml"],
        media_types: &["message/rfc822"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x78, 0x2D, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x2D, 0x69, 0x6E, 0x66, 0x6F,
                        0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
