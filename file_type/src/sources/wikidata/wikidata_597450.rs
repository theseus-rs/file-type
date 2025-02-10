use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_597450: FileType = FileType {
    file_format: &FileFormat {
        id: 597_450,
        source_type: SourceType::Wikidata,
        name: "Shorten",
        extensions: &["shn"],
        media_types: &["application/x-shorten"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x6A, 0x6B, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
