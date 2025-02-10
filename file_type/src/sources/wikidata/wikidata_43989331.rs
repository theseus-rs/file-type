use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_43989331: FileType = FileType {
    file_format: &FileFormat {
        id: 43_989_331,
        source_type: SourceType::Wikidata,
        name: "Quicken Data Format",
        extensions: &["qdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xAC, 0x9E, 0xBD, 0x8F, 0x00, 0x00]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
