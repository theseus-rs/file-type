use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860700: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_700,
        source_type: SourceType::Wikidata,
        name: "Golly Rule",
        extensions: &["rule"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x52, 0x55, 0x4C, 0x45, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
