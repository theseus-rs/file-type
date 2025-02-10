use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858343: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_343,
        source_type: SourceType::Wikidata,
        name: "E-Mail message (Var. 11)",
        extensions: &["eml"],
        media_types: &["message/rfc822"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x61, 0x74, 0x65, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
