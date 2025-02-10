use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_110869775: FileType = FileType {
    file_format: &FileFormat {
        id: 110_869_775,
        source_type: SourceType::Wikidata,
        name: "ARTstor Presentation",
        extensions: &["prs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4F, 0x49, 0x56]),
                        Token::Any(&[
                            &[Token::Literal(&[0x31])],
                            &[Token::Literal(&[0x32])],
                            &[Token::Literal(&[0x33])],
                            &[Token::Literal(&[0x34])],
                        ]),
                        Token::Literal(&[0x2E]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
