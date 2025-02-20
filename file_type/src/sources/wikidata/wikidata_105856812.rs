use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856812: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_812,
        source_type: SourceType::Wikidata,
        name: "Mind Games - Checkers saved game",
        extensions: &["gam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x68, 0x65, 0x63, 0x6B, 0x65, 0x72, 0x73, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
