use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27959894: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_894,
        source_type: SourceType::Wikidata,
        name: "Cubase project",
        extensions: &["cpr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x4E, 0x55, 0x4E, 0x44]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
