use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865805: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_805,
        source_type: SourceType::Wikidata,
        name: "PC64 flexible container format",
        extensions: &["p00", "r00", "s00", "u00"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x36, 0x34, 0x46, 0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
