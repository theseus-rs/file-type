use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762905: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_905,
        source_type: SourceType::Wikidata,
        name: "Xbox 360 LIVE container data file",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x49, 0x56, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
