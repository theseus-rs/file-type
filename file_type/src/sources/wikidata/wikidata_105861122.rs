use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861122: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_122,
        source_type: SourceType::Wikidata,
        name: "Magnetic Scrolls Collection Layout",
        extensions: &["lay"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x3C, 0x4C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
