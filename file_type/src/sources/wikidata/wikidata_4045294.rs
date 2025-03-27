use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4045294: FileType = FileType {
    file_format: &FileFormat {
        id: 4_045_294,
        source_type: SourceType::Wikidata,
        name: "New Executable",
        extensions: &["dll", "exe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x5A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x4E, 0x45]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
