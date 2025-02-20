use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3256475: FileType = FileType {
    file_format: &FileFormat {
        id: 3_256_475,
        source_type: SourceType::Wikidata,
        name: "Tiny Tafel",
        extensions: &["tt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
