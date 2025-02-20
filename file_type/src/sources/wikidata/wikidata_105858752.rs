use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858752: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_752,
        source_type: SourceType::Wikidata,
        name: "PAX password protected bitmap",
        extensions: &["pax"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
