use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854621: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_621,
        source_type: SourceType::Wikidata,
        name: "AllWebMenus project (v3.xx)",
        extensions: &["awm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x77, 0x6D, 0x33, 0x30, 0x30, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
